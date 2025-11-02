use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::info;

const DEEPGRAM_API_URL: &str = "https://api.deepgram.com/v1/listen";

#[derive(Debug, Deserialize)]
pub struct DeepgramResponse {
    pub results: DeepgramResults,
}

#[derive(Debug, Deserialize)]
pub struct DeepgramResults {
    pub channels: Vec<DeepgramChannel>,
}

#[derive(Debug, Deserialize)]
pub struct DeepgramChannel {
    pub alternatives: Vec<DeepgramAlternative>,
}

#[derive(Debug, Deserialize)]
pub struct DeepgramAlternative {
    pub transcript: String,
    pub confidence: f64,
}

/// Transcription result from Deepgram
#[derive(Debug, Clone)]
pub struct DeepgramResult {
    pub transcript: String,
    pub confidence: f64,
    pub word_count: usize,
}

impl DeepgramResult {
    /// Determine if audio is effectively jammed
    pub fn is_effectively_jammed(&self) -> bool {
        self.word_count == 0 || self.transcript.trim().is_empty() || self.confidence < 0.1
    }

    /// Get a quality score (0.0 = completely jammed, 1.0 = transcribed)
    pub fn quality_score(&self) -> f64 {
        if self.word_count == 0 {
            0.0
        } else {
            self.confidence
        }
    }
}

pub struct DeepgramClient {
    client: Client,
    api_key: String,
}

impl DeepgramClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Transcribe audio file with Deepgram
    pub async fn transcribe_file(&self, audio_path: &Path) -> Result<DeepgramResult> {
        info!("Transcribing audio file with Deepgram: {}", audio_path.display());

        // Read audio file
        let mut file = File::open(audio_path)
            .await
            .context("Failed to open audio file")?;

        let mut audio_data = Vec::new();
        file.read_to_end(&mut audio_data)
            .await
            .context("Failed to read audio file")?;

        // Make API request
        let response = self
            .client
            .post(DEEPGRAM_API_URL)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "audio/wav")
            .body(audio_data)
            .send()
            .await
            .context("Failed to send transcription request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Deepgram API error ({}): {}", status, error_text);
        }

        let deepgram_response: DeepgramResponse = response
            .json()
            .await
            .context("Failed to parse Deepgram response")?;

        let alternative = &deepgram_response.results.channels[0].alternatives[0];
        let transcript = alternative.transcript.trim().to_string();
        let confidence = alternative.confidence;
        let word_count = transcript.split_whitespace().count();

        let result = DeepgramResult {
            transcript: transcript.clone(),
            confidence,
            word_count,
        };

        info!("Deepgram transcription result:");
        info!("  Transcript: '{}'", transcript);
        info!("  Confidence: {:.2}", confidence);
        info!("  Word count: {}", word_count);
        info!("  Effectively jammed: {}", result.is_effectively_jammed());

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription_quality() {
        let good_result = DeepgramResult {
            transcript: "Hello, this is a clear transcription.".to_string(),
            confidence: 0.95,
            word_count: 6,
        };

        assert!(!good_result.is_effectively_jammed());
        assert!(good_result.quality_score() > 0.9);

        let jammed_result = DeepgramResult {
            transcript: "".to_string(),
            confidence: 0.0,
            word_count: 0,
        };

        assert!(jammed_result.is_effectively_jammed());
        assert!(jammed_result.quality_score() < 0.1);
    }
}
