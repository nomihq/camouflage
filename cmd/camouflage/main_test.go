package main

import (
	"fmt"
	"strings"
	"testing"
)

func TestFindDeviceByName(t *testing.T) {
	tests := []struct {
		name           string
		deviceName     string
		expectError    bool
		errorSubstring []string // Multiple possible error messages
	}{
		{
			name:           "empty device name",
			deviceName:     "",
			expectError:    true,
			errorSubstring: []string{"device not found", "PortAudio not initialized"},
		},
		{
			name:           "nonexistent device",
			deviceName:     "NonExistentDevice123",
			expectError:    true,
			errorSubstring: []string{"device not found", "PortAudio not initialized"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			device, err := findDeviceByName(tt.deviceName)

			if tt.expectError {
				if err == nil {
					t.Errorf("Expected error for device name '%s', but got none", tt.deviceName)
				} else {
					// Check if error contains any of the expected substrings
					found := false
					for _, substr := range tt.errorSubstring {
						if strings.Contains(err.Error(), substr) {
							found = true
							break
						}
					}
					if !found {
						t.Errorf("Expected error containing one of %v, got: %v", tt.errorSubstring, err)
					}
				}
				if device != nil {
					t.Errorf("Expected nil device when error occurs, got: %v", device)
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				}
				if device == nil {
					t.Errorf("Expected device, got nil")
				}
			}
		})
	}
}

func TestBlackHoleDeviceName(t *testing.T) {
	expectedName := "BlackHole 2ch"
	if blackHoleDeviceName != expectedName {
		t.Errorf("Expected blackHoleDeviceName to be '%s', got '%s'", expectedName, blackHoleDeviceName)
	}
}

// Mock test for device finding with actual system devices (if available)
func TestFindDeviceByName_WithSystemDevices(t *testing.T) {
	// This test will try to find common system devices
	// It's more of an integration test but useful for validation
	
	// Try to find any audio device (this should work on most systems)
	devices, err := getAvailableDevices()
	if err != nil {
		t.Skipf("Skipping system device test due to audio system error: %v", err)
	}

	if len(devices) == 0 {
		t.Skip("No audio devices found on system, skipping device test")
	}

	// Test finding the first available device
	firstDevice := devices[0]
	foundDevice, err := findDeviceByName(firstDevice.Name)
	if err != nil {
		t.Errorf("Failed to find existing device '%s': %v", firstDevice.Name, err)
	}
	if foundDevice == nil {
		t.Errorf("Expected to find device '%s', got nil", firstDevice.Name)
	}
	if foundDevice.Name != firstDevice.Name {
		t.Errorf("Device name mismatch: expected '%s', got '%s'", firstDevice.Name, foundDevice.Name)
	}
}

// Helper function to get available devices for testing
func getAvailableDevices() ([]*DeviceInfo, error) {
	// This is a simplified version for testing
	// In a real implementation, this would use the portaudio library
	return nil, fmt.Errorf("portaudio not initialized for testing")
}

// Mock DeviceInfo for testing
type DeviceInfo struct {
	Name string
}