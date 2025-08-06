package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gordonklaus/portaudio"
	"github.com/nomihq/camouflage/internal/ultrasonic"
)

type Config struct {
	Mode             string
	UltrasonicFreq   float64
	Duration         int
	BufferSize       int
	Verbose          bool
}

func main() {
	config := parseFlags()
	
	if config.Verbose {
		log.SetOutput(os.Stdout)
	} else {
		log.SetOutput(os.Stderr)
	}

	if err := portaudio.Initialize(); err != nil {
		log.Fatalf("Failed to initialize PortAudio: %v", err)
	}
	defer func() {
		if err := portaudio.Terminate(); err != nil {
			log.Printf("Warning: Failed to terminate PortAudio: %v", err)
		}
	}()

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
		BufferSize:     1024,
	}
	
	flag.StringVar(&config.Mode, "mode", "", "Mode: 'speaker' or 'system'")
	flag.Float64Var(&config.UltrasonicFreq, "freq", config.UltrasonicFreq, "Ultrasonic frequency in Hz (default: 25000)")
	flag.IntVar(&config.Duration, "duration", config.Duration, "Duration of ultrasonic loop in seconds (default: 1)")
	flag.IntVar(&config.BufferSize, "buffer-size", config.BufferSize, "Audio buffer size (default: 1024)")
	flag.BoolVar(&config.Verbose, "verbose", false, "Enable verbose logging")
	flag.Parse()
	
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

// --- Speaker Jammer ---

func runSpeakerJammer(ctx context.Context, config *Config) {
	log.Printf("Speaker Jammer mode activated (freq: %.0f Hz)", config.UltrasonicFreq)

	ultrasonicData := ultrasonic.GenerateSineWave(config.UltrasonicFreq, config.Duration)
	if len(ultrasonicData) == 0 {
		log.Fatal("Failed to generate ultrasonic data")
	}
	
	pos := 0

	stream, err := portaudio.OpenDefaultStream(0, 1, ultrasonic.SampleRate, 0, func(out []float32) {
		for i := range out {
			out[i] = float32(ultrasonicData[pos])
			pos = (pos + 1) % len(ultrasonicData)
		}
	})
	if err != nil {
		log.Fatalf("Failed to open audio stream: %v", err)
	}
	defer func() {
		if err := stream.Close(); err != nil {
			log.Printf("Warning: Failed to close audio stream: %v", err)
		}
	}()

	if err := stream.Start(); err != nil {
		log.Fatalf("Failed to start audio stream: %v", err)
	}
	defer func() {
		if err := stream.Stop(); err != nil {
			log.Printf("Warning: Failed to stop audio stream: %v", err)
		}
	}()

	log.Println("Playing ultrasonic frequency. Press Ctrl+C to stop.")
	<-ctx.Done()
	log.Println("Speaker jammer stopped.")
}

// --- System Jammer ---

func runSystemJammer(ctx context.Context, config *Config) {
	log.Printf("On-System Jammer mode activated (freq: %.0f Hz)", config.UltrasonicFreq)

	if !checkForBlackHole() {
		log.Println("BlackHole not found, attempting installation...")
		if err := installBlackHole(); err != nil {
			log.Fatalf("Error installing BlackHole: %v", err)
		}
		// Give the system time to recognize the new device
		time.Sleep(2 * time.Second)
	}

	log.Println("Starting audio processing...")
	log.Println("Please set 'BlackHole 2ch' as your system's audio output.")

	in, out, err := openSystemJammerStreams(config)
	if err != nil {
		log.Fatalf("Failed to open system jammer streams: %v", err)
	}
	defer func() {
		if err := in.Close(); err != nil {
			log.Printf("Warning: Failed to close input stream: %v", err)
		}
		if err := out.Close(); err != nil {
			log.Printf("Warning: Failed to close output stream: %v", err)
		}
	}()

	if err := in.Start(); err != nil {
		log.Fatalf("Failed to start input stream: %v", err)
	}
	if err := out.Start(); err != nil {
		log.Fatalf("Failed to start output stream: %v", err)
	}
	defer func() {
		if err := in.Stop(); err != nil {
			log.Printf("Warning: Failed to stop input stream: %v", err)
		}
		if err := out.Stop(); err != nil {
			log.Printf("Warning: Failed to stop output stream: %v", err)
		}
	}()

	log.Println("Audio loop running. Press Ctrl+C to stop.")
	<-ctx.Done()
	log.Println("System jammer stopped.")
}

func openSystemJammerStreams(config *Config) (*portaudio.Stream, *portaudio.Stream, error) {
	blackhole, err := findDeviceByName(blackHoleDeviceName)
	if err != nil {
		return nil, nil, fmt.Errorf("could not find BlackHole device: %w", err)
	}

	defaultOut, err := portaudio.DefaultOutputDevice()
	if err != nil {
		return nil, nil, fmt.Errorf("could not get default output device: %w", err)
	}

	ultrasonicData := ultrasonic.GenerateSineWave(config.UltrasonicFreq, config.Duration)
	if len(ultrasonicData) == 0 {
		return nil, nil, fmt.Errorf("failed to generate ultrasonic data")
	}
	ultrasonicPos := 0

	// This buffer will pass audio from the input stream to the output stream.
	buffer := make([]float32, config.BufferSize)

	// Input stream (from BlackHole)
	inputStream, err := portaudio.OpenStream(portaudio.StreamParameters{
		Input: portaudio.StreamDeviceParameters{Device: blackhole, Channels: 2},
		SampleRate:      ultrasonic.SampleRate,
		FramesPerBuffer: len(buffer) / 2,
	}, func(in []float32) {
		if len(in) <= len(buffer) {
			copy(buffer[:len(in)], in)
		} else {
			copy(buffer, in[:len(buffer)])
		}
	})
	if err != nil {
		return nil, nil, fmt.Errorf("could not open input stream: %w", err)
	}

	// Output stream (to default speakers)
	outputStream, err := portaudio.OpenStream(portaudio.StreamParameters{
		Output: portaudio.StreamDeviceParameters{Device: defaultOut, Channels: 2},
		SampleRate:       ultrasonic.SampleRate,
		FramesPerBuffer:  len(buffer) / 2,
	}, func(out []float32) {
		for i := range out {
			var inputSample float32
			if i < len(buffer) {
				inputSample = buffer[i]
			}

			// Mix input audio with ultrasonic signal
			ultrasonicSample := float32(ultrasonicData[ultrasonicPos])
			mixedSample := inputSample + ultrasonicSample*0.1 // Reduce ultrasonic volume

			// Soft clipping to avoid harsh distortion
			mixedSample = softClip(mixedSample)
			out[i] = mixedSample

			// Advance ultrasonic position for the next sample
			if i%2 == 1 { // Advance only once per stereo frame
				ultrasonicPos = (ultrasonicPos + 1) % len(ultrasonicData)
			}
		}
	})
	if err != nil {
		inputStream.Close()
		return nil, nil, fmt.Errorf("could not open output stream: %w", err)
	}

	return inputStream, outputStream, nil
}

// softClip applies a soft clipping function to prevent harsh distortion
func softClip(sample float32) float32 {
	if sample > 1.0 {
		return 0.9 + 0.1*float32(1.0/(1.0+float64(sample-1.0)))
	} else if sample < -1.0 {
		return -0.9 - 0.1*float32(1.0/(1.0+float64(-sample-1.0)))
	}
	return sample
}

func findDeviceByName(name string) (*portaudio.DeviceInfo, error) {
	devices, err := portaudio.Devices()
	if err != nil {
		return nil, err
	}
	for _, device := range devices {
		if device.Name == name {
			return device, nil
		}
	}
	return nil, fmt.Errorf("device not found: %s", name)
}