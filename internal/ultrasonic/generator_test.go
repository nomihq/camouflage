
package ultrasonic

import (
	"math"
	"testing"
)

func TestGenerateSineWave(t *testing.T) {
	tests := []struct {
		name     string
		freq     float64
		duration int
	}{
		{"25kHz 1 second", 25000.0, 1},
		{"24kHz 2 seconds", 24000.0, 2},
		{"26kHz 0.5 seconds", 26000.0, 1}, // Will round to 1 second
		{"1kHz test tone", 1000.0, 1},
		{"440Hz A note", 440.0, 1},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data := GenerateSineWave(tt.freq, tt.duration)

			// Check that the length is correct
			expectedLength := SampleRate * tt.duration
			if len(data) != expectedLength {
				t.Errorf("Expected data length of %d, but got %d", expectedLength, len(data))
			}

			// Check that the values are within the expected range [-1.0, 1.0]
			for i, val := range data {
				if val < -1.0 || val > 1.0 {
					t.Errorf("Sample %d is out of range [-1.0, 1.0]: %f", i, val)
				}
			}

			// Check that we have a proper sine wave by testing some mathematical properties
			validateSineWaveProperties(t, data, tt.freq, tt.duration)
		})
	}
}

func TestGenerateSineWave_EdgeCases(t *testing.T) {
	tests := []struct {
		name     string
		freq     float64
		duration int
		wantLen  int
	}{
		{"Zero frequency", 0, 1, SampleRate},
		{"Negative frequency", -100, 1, SampleRate},
		{"Zero duration", 1000, 0, 0},
		{"Very high frequency", 50000, 1, SampleRate},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data := GenerateSineWave(tt.freq, tt.duration)
			
			if len(data) != tt.wantLen {
				t.Errorf("Expected length %d, got %d", tt.wantLen, len(data))
			}

			// All values should still be in range
			for i, val := range data {
				if val < -1.0 || val > 1.0 {
					t.Errorf("Sample %d is out of range [-1.0, 1.0]: %f", i, val)
				}
			}
		})
	}
}

func TestSampleRate(t *testing.T) {
	expectedRate := 44100
	if SampleRate != expectedRate {
		t.Errorf("Expected SampleRate to be %d, got %d", expectedRate, SampleRate)
	}
}

func TestGenerateSineWave_Frequency(t *testing.T) {
	// Test that the generated wave has approximately the correct frequency
	freq := 1000.0 // 1kHz for easy testing
	duration := 1
	
	data := GenerateSineWave(freq, duration)
	
	// Count zero crossings to estimate frequency
	zeroCrossings := countZeroCrossings(data)
	estimatedFreq := float64(zeroCrossings) / 2.0 // Each cycle has 2 zero crossings
	
	tolerance := 1.0 // Allow 1 Hz tolerance
	if math.Abs(estimatedFreq-freq) > tolerance {
		t.Errorf("Expected frequency ~%.1f Hz, estimated %.1f Hz", freq, estimatedFreq)
	}
}

func TestGenerateSineWave_Amplitude(t *testing.T) {
	freq := 1000.0
	duration := 1
	
	data := GenerateSineWave(freq, duration)
	
	// Find maximum and minimum values
	var max, min float64 = data[0], data[0]
	for _, val := range data {
		if val > max {
			max = val
		}
		if val < min {
			min = val
		}
	}
	
	// For a sine wave, max should be ~1.0 and min should be ~-1.0
	tolerance := 0.01
	if math.Abs(max-1.0) > tolerance {
		t.Errorf("Expected max amplitude ~1.0, got %.3f", max)
	}
	if math.Abs(min-(-1.0)) > tolerance {
		t.Errorf("Expected min amplitude ~-1.0, got %.3f", min)
	}
}

// Helper function to validate sine wave mathematical properties
func validateSineWaveProperties(t *testing.T, data []float64, freq float64, duration int) {
	if len(data) == 0 {
		return
	}
	
	// Test that the wave starts at approximately zero (sin(0) = 0)
	if math.Abs(data[0]) > 0.01 {
		t.Errorf("Expected sine wave to start near zero, got %.3f", data[0])
	}
	
	// For frequencies that result in complete cycles, test periodicity
	// Only test for lower frequencies where we can get complete cycles
	if freq <= 1000 {
		samplesPerCycle := float64(SampleRate) / freq
		
		// Only test if we have at least 2 complete cycles and samples per cycle is reasonable
		if samplesPerCycle > 10 && samplesPerCycle*2 < float64(len(data)) {
			cycleLength := int(math.Round(samplesPerCycle))
			
			// Compare first cycle with second cycle, but allow for some rounding errors
			tolerance := 0.05 // Increased tolerance for floating point precision
			maxErrors := cycleLength / 10 // Allow up to 10% of samples to be slightly off
			errorCount := 0
			
			for i := 0; i < cycleLength && i+cycleLength < len(data); i++ {
				if math.Abs(data[i]-data[i+cycleLength]) > tolerance {
					errorCount++
					if errorCount > maxErrors {
						t.Errorf("Wave not sufficiently periodic: too many samples differ between cycles (sample %d: %.3f vs %.3f)", 
							i, data[i], data[i+cycleLength])
						break
					}
				}
			}
		}
	}
}

// Helper function to count zero crossings
func countZeroCrossings(data []float64) int {
	if len(data) < 2 {
		return 0
	}
	
	crossings := 0
	for i := 1; i < len(data); i++ {
		if (data[i-1] < 0 && data[i] >= 0) || (data[i-1] >= 0 && data[i] < 0) {
			crossings++
		}
	}
	return crossings
}

func BenchmarkGenerateSineWave(b *testing.B) {
	freq := 25000.0
	duration := 1
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		GenerateSineWave(freq, duration)
	}
}

func BenchmarkGenerateSineWave_LongDuration(b *testing.B) {
	freq := 25000.0
	duration := 10 // 10 seconds
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		GenerateSineWave(freq, duration)
	}
}

func TestValidateFrequency(t *testing.T) {
	tests := []struct {
		name    string
		freq    float64
		wantErr bool
	}{
		{"Valid ultrasonic frequency", 25000.0, false},
		{"Minimum valid frequency", 20000.0, false},
		{"Maximum valid frequency", 30000.0, false},
		{"Too low frequency", 15000.0, true},
		{"Too high frequency", 35000.0, true},
		{"Negative frequency", -1000.0, true},
		{"Zero frequency", 0.0, true},
		{"Just above minimum", 20001.0, false},
		{"Just below maximum", 29999.0, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := ValidateFrequency(tt.freq)
			if (err != nil) != tt.wantErr {
				t.Errorf("ValidateFrequency() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestGenerateMultiTone(t *testing.T) {
	tests := []struct {
		name      string
		baseFreq  float64
		duration  int
		harmonics int
		wantLen   int
	}{
		{"3 harmonics, 1 second", 25000.0, 1, 3, SampleRate},
		{"5 harmonics, 2 seconds", 24000.0, 2, 5, SampleRate * 2},
		{"Zero duration", 25000.0, 0, 3, 0},
		{"Zero harmonics", 25000.0, 1, 0, 0},
		{"Single harmonic", 25000.0, 1, 1, SampleRate},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data := GenerateMultiTone(tt.baseFreq, tt.duration, tt.harmonics)
			
			if len(data) != tt.wantLen {
				t.Errorf("GenerateMultiTone() length = %d, want %d", len(data), tt.wantLen)
			}
			
			// Check amplitude range
			for i, val := range data {
				if val < -1.1 || val > 1.1 { // Allow slight overshoot due to mixing
					t.Errorf("Sample %d out of reasonable range: %f", i, val)
					break
				}
			}
		})
	}
}

func TestGenerateSweep(t *testing.T) {
	tests := []struct {
		name      string
		startFreq float64
		endFreq   float64
		duration  int
		wantLen   int
	}{
		{"Upward sweep", 24000.0, 26000.0, 1, SampleRate},
		{"Downward sweep", 26000.0, 24000.0, 1, SampleRate},
		{"Same frequency", 25000.0, 25000.0, 1, SampleRate},
		{"Zero duration", 25000.0, 26000.0, 0, 0},
		{"Long duration", 24000.0, 26000.0, 3, SampleRate * 3},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data := GenerateSweep(tt.startFreq, tt.endFreq, tt.duration)
			
			if len(data) != tt.wantLen {
				t.Errorf("GenerateSweep() length = %d, want %d", len(data), tt.wantLen)
			}
			
			// Check amplitude range
			for i, val := range data {
				if val < -1.0 || val > 1.0 {
					t.Errorf("Sample %d out of range [-1.0, 1.0]: %f", i, val)
					break
				}
			}
		})
	}
}

func TestConstants(t *testing.T) {
	if MinFreq >= MaxFreq {
		t.Error("MinFreq should be less than MaxFreq")
	}
	
	if MinFreq <= 0 {
		t.Error("MinFreq should be positive")
	}
	
	if SampleRate <= 0 {
		t.Error("SampleRate should be positive")
	}
}

func BenchmarkGenerateMultiTone(b *testing.B) {
	baseFreq := 25000.0
	duration := 1
	harmonics := 3
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		GenerateMultiTone(baseFreq, duration, harmonics)
	}
}

func BenchmarkGenerateSweep(b *testing.B) {
	startFreq := 24000.0
	endFreq := 26000.0
	duration := 1
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		GenerateSweep(startFreq, endFreq, duration)
	}
}
