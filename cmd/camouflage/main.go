
package main

import (
	"flag"
	"fmt"
	"math"
	"os"

	"github.com/hajimehoshi/oto/v2"
	"github.com/nullswan/camouflage/internal/ultrasonic"
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
	fmt.Println("Speaker Jammer mode activated. Playing ultrasonic frequency.")

	// Generate a 25kHz sine wave for 1 second (it will be looped).
	ultrasonicData := ultrasonic.GenerateSineWave(25000, 1)

	// Oto uses a context to manage the audio driver.
	opts := &oto.NewContextOptions{
		SampleRate:   ultrasonic.SampleRate,
		ChannelCount: 1, // Mono
		Format:       oto.FormatFloat32LE, // 32-bit little-endian float
	}

	otoCtx, ready, err := oto.NewContextWithOptions(opts)
	if err != nil {
		panic("oto.NewContext failed: " + err.Error())
	}
	// Wait for the context to be ready.
	<-ready

	// Create a player for the ultrasonic sound.
	player := otoCtx.NewPlayer(newInfiniteReader(ultrasonicData))
	player.Play()

	fmt.Println("Playing. Press Ctrl+C to stop.")
	// Keep the program running to play the sound indefinitely.
	select {}
}

// infiniteReader is a helper to loop the audio data.
type infiniteReader struct {
	data []float32
	pos  int
}

func newInfiniteReader(data []float64) *infiniteReader {
	// Convert to float32 for Oto
	float32Data := make([]float32, len(data))
	for i, v := range data {
		float32Data[i] = float32(v)
	}
	return &infiniteReader{data: float32Data}
}

func (r *infiniteReader) Read(buf []byte) (int, error) {
	bytesPerSample := 4 // 32-bit float
	numSamplesToWrite := len(buf) / bytesPerSample

	for i := 0; i < numSamplesToWrite; i++ {
		// Get the sample from our data, looping if necessary.
		sample := r.data[r.pos]
		r.pos = (r.pos + 1) % len(r.data)

		// Convert the float32 sample to bytes (Little Endian).
		b := math.Float32bits(sample)
		buf[i*4] = byte(b)
		buf[i*4+1] = byte(b >> 8)
		buf[i*4+2] = byte(b >> 16)
		buf[i*4+3] = byte(b >> 24)
	}

	return len(buf), nil
}

// runSystemJammer will contain the logic for the on-system jammer.
func runSystemJammer() {
	fmt.Println("On-System Jammer mode activated.")

	// 1. Check for and install BlackHole if necessary.
	if !checkForBlackHole() {
		if err := installBlackHole(); err != nil {
			fmt.Fprintf(os.Stderr, "Error during BlackHole installation: %v\n", err)
			os.Exit(1)
		}
	}

	// 2. Now that we know BlackHole is installed, we can proceed with audio routing.
	fmt.Println("BlackHole is ready. Starting audio processing...")
	// TODO: Implement audio capture from BlackHole, mixing, and output.
	// This will involve using oto to read from BlackHole and write to the default output.

	fmt.Println("System Jammer logic is not fully implemented yet. Exiting.")
}

