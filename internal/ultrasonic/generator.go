
package ultrasonic

import (
	"fmt"
	"math"
)

// SampleRate is the number of samples per second.
const SampleRate = 44100

// MinFreq is the minimum recommended frequency for ultrasonic jamming
const MinFreq = 20000.0

// MaxFreq is the maximum recommended frequency for ultrasonic jamming
const MaxFreq = 30000.0

// GenerateSineWave generates a sine wave at a given frequency.
// It returns a slice of float64 values, representing the raw audio data.
func GenerateSineWave(freq float64, durationSec int) []float64 {
	if durationSec <= 0 {
		return []float64{}
	}
	
	numSamples := SampleRate * durationSec
	if numSamples <= 0 {
		return []float64{}
	}
	
	data := make([]float64, numSamples)

	for i := 0; i < numSamples; i++ {
		angle := 2 * math.Pi * freq * float64(i) / SampleRate
		data[i] = math.Sin(angle)
	}

	return data
}

// ValidateFrequency checks if a frequency is suitable for ultrasonic jamming
func ValidateFrequency(freq float64) error {
	if freq < 0 {
		return fmt.Errorf("frequency cannot be negative: %.1f", freq)
	}
	if freq < MinFreq {
		return fmt.Errorf("frequency %.1f Hz is too low for effective ultrasonic jamming (minimum: %.1f Hz)", freq, MinFreq)
	}
	if freq > MaxFreq {
		return fmt.Errorf("frequency %.1f Hz is too high and may be ineffective (maximum: %.1f Hz)", freq, MaxFreq)
	}
	return nil
}

// GenerateMultiTone generates multiple sine waves at different frequencies
// and mixes them together for more effective jamming
func GenerateMultiTone(baseFreq float64, durationSec int, harmonics int) []float64 {
	if durationSec <= 0 || harmonics <= 0 {
		return []float64{}
	}
	
	numSamples := SampleRate * durationSec
	if numSamples <= 0 {
		return []float64{}
	}
	
	data := make([]float64, numSamples)
	
	for i := 0; i < numSamples; i++ {
		var sample float64
		for h := 1; h <= harmonics; h++ {
			freq := baseFreq + float64(h-1)*100 // Spread frequencies 100Hz apart
			angle := 2 * math.Pi * freq * float64(i) / SampleRate
			amplitude := 1.0 / float64(harmonics) // Normalize amplitude
			sample += amplitude * math.Sin(angle)
		}
		data[i] = sample
	}
	
	return data
}

// GenerateSweep generates a frequency sweep from startFreq to endFreq
// This can be more effective against adaptive noise cancellation
func GenerateSweep(startFreq, endFreq float64, durationSec int) []float64 {
	if durationSec <= 0 {
		return []float64{}
	}
	
	numSamples := SampleRate * durationSec
	if numSamples <= 0 {
		return []float64{}
	}
	
	data := make([]float64, numSamples)
	freqRange := endFreq - startFreq
	
	for i := 0; i < numSamples; i++ {
		// Linear frequency sweep
		progress := float64(i) / float64(numSamples-1)
		currentFreq := startFreq + freqRange*progress
		
		angle := 2 * math.Pi * currentFreq * float64(i) / SampleRate
		data[i] = math.Sin(angle)
	}
	
	return data
}
