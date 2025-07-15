
package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	// Define a command-line flag for the mode
	mode := flag.String("mode", "", "Mode to run the application in: 'speaker' or 'system'")
	flag.Parse()

	// Check the mode and run the corresponding function
	switch *mode {
	case "speaker":
		runSpeakerJammer()
	case "system":
		runSystemJammer()
	default:
		fmt.Println("Error: Invalid or no mode specified. Use --mode speaker or --mode system")
		os.Exit(1)
	}
}

// runSpeakerJammer will contain the logic for the speaker jammer.
func runSpeakerJammer() {
	fmt.Println("Speaker Jammer mode activated.")
	// TODO: Implement ultrasonic generation and output to speakers
}

// runSystemJammer will contain the logic for the on-system jammer.
func runSystemJammer() {
	fmt.Println("On-System Jammer mode activated.")
	// TODO: Implement virtual audio device and ultrasonic injection
}

