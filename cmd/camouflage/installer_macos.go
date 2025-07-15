//go:build darwin
// +build darwin

package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
)

const blackHoleDeviceName = "BlackHole 2ch"

// checkForBlackHole checks if the BlackHole audio device is installed.
func checkForBlackHole() bool {
	// We can check for the presence of the BlackHole driver directory.
	// This is a reliable way to see if it's installed without parsing command output.
	const blackHoleDriverPath = "/Library/Audio/Plug-Ins/HAL/BlackHole.driver"
	if _, err := os.Stat(blackHoleDriverPath); err == nil {
		fmt.Println("BlackHole audio driver is already installed.")
		return true
	}
	return false
}

// installBlackHole downloads and installs the BlackHole driver.
func installBlackHole() error {
	fmt.Println("BlackHole driver not found. Attempting to install...")

	// 1. Download the .pkg installer
	// In a real implementation, we would fetch the latest release URL from GitHub API.
	// For now, we'll use a known URL for a specific version.
	const pkgUrl = "https://github.com/ExistentialAudio/BlackHole/releases/download/v0.6.0/BlackHole-2ch.v0.6.0.pkg"
	const pkgPath = "/tmp/BlackHole.pkg"

	fmt.Printf("Downloading installer from %s...\n", pkgUrl)
	resp, err := http.Get(pkgUrl)
	if err != nil {
		return fmt.Errorf("failed to download installer: %w", err)
	}
	defer resp.Body.Close()

	out, err := os.Create(pkgPath)
	if err != nil {
		return fmt.Errorf("failed to create temporary installer file: %w", err)
	}
	defer out.Close()

	_, err = io.Copy(out, resp.Body)
	if err != nil {
		return fmt.Errorf("failed to save installer: %w", err)
	}

	// 2. Run the installer with sudo
	fmt.Println("\n--- ACTION REQUIRED ---")
	fmt.Println("To protect your calls, Camouflage needs to install the 'BlackHole' virtual audio driver.")
	fmt.Println("This requires your administrator password to proceed.")
	fmt.Println("-----------------------")

	cmd := exec.Command("sudo", "installer", "-pkg", pkgPath, "-target", "/")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("installer command failed. Please try again or install BlackHole manually. Error: %w", err)
	}

	// 3. Clean up
	fmt.Println("Installation successful. Cleaning up...")
	return os.Remove(pkgPath)
}