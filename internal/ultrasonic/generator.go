
package ultrasonic

import (
	"math"
)

// SampleRate is the number of samples per second.
const SampleRate = 44100

// GenerateSineWave generates a sine wave at a given frequency.
// It returns a slice of float64 values, representing the raw audio data.
func GenerateSineWave(freq float64, durationSec int) []float64 {
	numSamples := SampleRate * durationSec
	data := make([]float64, numSamples)

	for i := 0; i < numSamples; i++ {
		angle := 2 * math.Pi * freq * float64(i) / SampleRate
		data[i] = math.Sin(angle)
	}

	return data
}
