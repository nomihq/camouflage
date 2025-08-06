package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/gordonklaus/portaudio"
	"github.com/nomihq/camouflage/internal/ultrasonic"
)

func main() {
	portaudio.Initialize()
	defer portaudio.Terminate()

	mode := flag.String("mode", "", "Mode: 'speaker' or 'system'")
	flag.Parse()

	switch *mode {
	case "speaker":
		runSpeakerJammer()
	case "system":
		runSystemJammer()
	default:
		fmt.Println("Error: Use --mode speaker or --mode system")
		os.Exit(1)
	}
}

// --- Speaker Jammer ---

func runSpeakerJammer() {
	fmt.Println("Speaker Jammer mode activated.")

	ultrasonicData := ultrasonic.GenerateSineWave(25000, 1) // 1 sec loop
	pos := 0

	stream, err := portaudio.OpenDefaultStream(0, 1, ultrasonic.SampleRate, 0, func(out []float32) {
		for i := range out {
			out[i] = float32(ultrasonicData[pos])
			pos = (pos + 1) % len(ultrasonicData)
		}
	})
	if err != nil {
		panic(err)
	}
	defer stream.Close()

	if err := stream.Start(); err != nil {
		panic(err)
	}

	fmt.Println("Playing ultrasonic frequency. Press Ctrl+C to stop.")
	select {}
}

// --- System Jammer ---

func runSystemJammer() {
	fmt.Println("On-System Jammer mode activated.")

	if !checkForBlackHole() {
		if err := installBlackHole(); err != nil {
			fmt.Fprintf(os.Stderr, "Error installing BlackHole: %v\n", err)
			os.Exit(1)
		}
	}

	fmt.Println("Starting audio processing...")
	fmt.Println("Please set 'BlackHole 2ch' as your system's audio output.")

	in, out := openSystemJammerStreams()
	defer in.Close()
	defer out.Close()

	if err := in.Start(); err != nil {
		panic(err)
	}
	if err := out.Start(); err != nil {
		panic(err)
	}

	fmt.Println("Audio loop running. Press Ctrl+C to stop.")
	select {}
}

func openSystemJammerStreams() (*portaudio.Stream, *portaudio.Stream) {
	blackhole, err := findDeviceByName(blackHoleDeviceName)
	if err != nil {
		panic("Could not find BlackHole device. Is it installed?")
	}

	defaultOut, err := portaudio.DefaultOutputDevice()
	if err != nil {
		panic(err)
	}

	ultrasonicData := ultrasonic.GenerateSineWave(25000, 1)
	ultrasonicPos := 0

	// This buffer will pass audio from the input stream to the output stream.
	buffer := make([]float32, 1024)

	// Input stream (from BlackHole)
	inputStream, err := portaudio.OpenStream(portaudio.StreamParameters{
		Input: portaudio.StreamDeviceParameters{Device: blackhole, Channels: 2},
		SampleRate:      ultrasonic.SampleRate,
		FramesPerBuffer: len(buffer) / 2,
	}, func(in []float32) {
		copy(buffer, in)
	})
	if err != nil {
		panic(err)
	}

	// Output stream (to default speakers)
	outputStream, err := portaudio.OpenStream(portaudio.StreamParameters{
		Output: portaudio.StreamDeviceParameters{Device: defaultOut, Channels: 2},
		SampleRate:       ultrasonic.SampleRate,
		FramesPerBuffer:  len(buffer) / 2,
	}, func(out []float32) {
		for i := range out {
			// Mix input audio with ultrasonic signal
			mixedSample := buffer[i] + float32(ultrasonicData[ultrasonicPos])

			// Clip the signal
			if mixedSample > 1.0 {
				mixedSample = 1.0
			} else if mixedSample < -1.0 {
				mixedSample = -1.0
			}
			out[i] = mixedSample

			// Advance ultrasonic position for the next sample
			if i%2 == 1 { // Advance only once per stereo frame
				ultrasonicPos = (ultrasonicPos + 1) % len(ultrasonicData)
			}
		}
	})
	if err != nil {
		panic(err)
	}

	return inputStream, outputStream
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