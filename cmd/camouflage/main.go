
package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"math"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/hajimehoshi/oto/v2"
	"github.com/nomihq/camouflage/internal/ultrasonic"
)

type Config struct {
	Mode           string
	UltrasonicFreq float64
	Duration       int
	Channels       int
	Verbose        bool
}

func main() {
	config := parseFlags()
	
	if config.Verbose {
		log.SetOutput(os.Stdout)
	} else {
		log.SetOutput(os.Stderr)
	}

	ctx, cancel := setupSignalHandling()
	defer cancel()

	switch config.Mode {
	case "speaker":
		runSpeakerJammer(ctx, config)
	case "system":
		runSystemJammer(ctx, config)
	default:
		fmt.Fprintf(os.Stderr, "Error: Use --mode speaker or --mode system\n")
		os.Exit(1)
	}
}

func parseFlags() *Config {
	config := &Config{
		UltrasonicFreq: 25000.0,
		Duration:       1,
		Channels:       1,
	}
	
	flag.StringVar(&config.Mode, "mode", "", "Mode: 'speaker' or 'system'")
	flag.Float64Var(&config.UltrasonicFreq, "freq", config.UltrasonicFreq, "Ultrasonic frequency in Hz (default: 25000)")
	flag.IntVar(&config.Duration, "duration", config.Duration, "Duration of ultrasonic loop in seconds (default: 1)")
	flag.IntVar(&config.Channels, "channels", config.Channels, "Number of audio channels (default: 1)")
	flag.BoolVar(&config.Verbose, "verbose", false, "Enable verbose logging")
	flag.Parse()
	
	// Validate configuration
	if err := ultrasonic.ValidateFrequency(config.UltrasonicFreq); err != nil && config.Verbose {
		log.Printf("Warning: %v", err)
	}
	
	return config
}

func setupSignalHandling() (context.Context, context.CancelFunc) {
	ctx, cancel := context.WithCancel(context.Background())
	
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	
	go func() {
		<-sigChan
		log.Println("\nShutdown signal received, stopping...")
		cancel()
	}()
	
	return ctx, cancel
}

// runSpeakerJammer contains the logic for the speaker jammer using oto/v2.
func runSpeakerJammer(ctx context.Context, config *Config) {
	log.Printf("Speaker Jammer mode activated (freq: %.0f Hz, oto/v2 backend)", config.UltrasonicFreq)

	// Generate ultrasonic wave data
	ultrasonicData := ultrasonic.GenerateSineWave(config.UltrasonicFreq, config.Duration)
	if len(ultrasonicData) == 0 {
		log.Fatal("Failed to generate ultrasonic data")
	}

	// Oto uses a context to manage the audio driver.
	opts := &oto.NewContextOptions{
		SampleRate:   ultrasonic.SampleRate,
		ChannelCount: config.Channels,
		Format:       oto.FormatFloat32LE, // 32-bit little-endian float
	}

	otoCtx, ready, err := oto.NewContextWithOptions(opts)
	if err != nil {
		log.Fatalf("Failed to create oto context: %v", err)
	}
	// Note: oto/v2 Context doesn't have a Close method

	// Wait for the context to be ready.
	select {
	case <-ready:
		if config.Verbose {
			log.Println("Audio context ready")
		}
	case <-ctx.Done():
		log.Println("Cancelled before audio context was ready")
		return
	case <-time.After(5 * time.Second):
		log.Fatal("Timeout waiting for audio context to be ready")
	}

	// Create a player for the ultrasonic sound.
	player := otoCtx.NewPlayer(newInfiniteReader(ultrasonicData))
	defer func() {
		if err := player.Close(); err != nil {
			log.Printf("Warning: Failed to close player: %v", err)
		}
	}()

	player.Play()

	log.Println("Playing ultrasonic frequency. Press Ctrl+C to stop.")
	<-ctx.Done()
	log.Println("Speaker jammer stopped.")
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

// runSystemJammer contains the logic for the on-system jammer.
func runSystemJammer(ctx context.Context, config *Config) {
	log.Printf("On-System Jammer mode activated (freq: %.0f Hz)", config.UltrasonicFreq)
	log.Println("Note: System jamming not fully implemented with oto/v2 backend.")
	log.Println("Consider using the portaudio-based implementation for full system jamming.")
	
	// For now, just run speaker jammer as a placeholder
	runSpeakerJammer(ctx, config)
}

