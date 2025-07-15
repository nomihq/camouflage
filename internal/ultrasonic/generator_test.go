
package ultrasonic

import (
	"testing"
)

func TestGenerateSineWave(t *testing.T) {
	// Test case: Generate a 1-second, 25kHz sine wave
	freq := 25000.0
	duration := 1

	data := GenerateSineWave(freq, duration)

	// Check that the length is correct
	expectedLength := SampleRate * duration
	if len(data) != expectedLength {
		t.Errorf("Expected data length of %d, but got %d", expectedLength, len(data))
	}

	// Check that the values are within the expected range [-1.0, 1.0]
	for i, val := range data {
		if val < -1.0 || val > 1.0 {
			t.Errorf("Sample %d is out of range [-1.0, 1.0]: %f", i, val)
		}
	}
}
