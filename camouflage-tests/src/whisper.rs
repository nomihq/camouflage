use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::info;

const WHISPER_API_URL: &str = "https://api.openai.com/v1/audio/transcriptions";

#[derive(Debug, Deserialize)]
pub struct WhisperResponse {
    pub text: String,
}

/// Transcription result with quality metrics
#[derive(Debug, Clone)]
pub struct WhisperResult {
    pub transcript: String,
    pub word_count: usize,
    pub is_jammed: bool,
}

impl WhisperResult {
    /// Determine if audio is effectively jammed
    pub fn is_effectively_jammed(&self) -> bool {
        self.word_count == 0 || self.transcript.trim().is_empty()
    }

    /// Get a quality score (0.0 = completely jammed, 1.0 = transcribed)
    pub fn quality_score(&self) -> f64 {
        if self.word_count == 0 {
            0.0
        } else {
            1.0 // Whisper doesn't provide confidence
        }
    }
}

pub struct WhisperClient {
    client: Client,
    api_key: String,
}

impl WhisperClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Transcribe audio file with Whisper
    pub async fn transcribe_file(&self, audio_path: &Path) -> Result<WhisperResult> {
        info!("Transcribing audio file with Whisper: {}", audio_path.display());

        // Read audio file
        let mut file = File::open(audio_path)
            .await
            .context("Failed to open audio file")?;

        let mut audio_data = Vec::new();
        file.read_to_end(&mut audio_data)
            .await
            .context("Failed to read audio file")?;

        // Determine filename
        let filename = audio_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("audio.wav");

        // Create multipart form
        let part = reqwest::multipart::Part::bytes(audio_data)
            .file_name(filename.to_string())
            .mime_str("audio/wav")?;

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("model", "whisper-1");

        // Make API request
        let response = self
            .client
            .post(WHISPER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .context("Failed to send transcription request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Whisper API error ({}): {}", status, error_text);
        }

        let whisper_response: WhisperResponse = response
            .json()
            .await
            .context("Failed to parse Whisper response")?;

        let transcript = whisper_response.text.trim().to_string();
        let word_count = transcript.split_whitespace().count();

        let result = WhisperResult {
            transcript: transcript.clone(),
            word_count,
            is_jammed: false,
        };

        info!("Whisper transcription result:");
        info!("  Transcript: '{}'", transcript);
        info!("  Word count: {}", word_count);
        info!("  Effectively jammed: {}", result.is_effectively_jammed());

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    #[ignore]
    async fn test_whisper_transcription() {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let _client = WhisperClient::new(api_key);

        // This test requires a valid audio file
    }

    #[test]
    fn test_transcription_quality() {
        let good_result = WhisperResult {
            transcript: "Hello, this is a clear transcription.".to_string(),
            word_count: 6,
            is_jammed: false,
        };

        assert!(!good_result.is_effectively_jammed());
        assert!(good_result.quality_score() > 0.9);

        let jammed_result = WhisperResult {
            transcript: "".to_string(),
            word_count: 0,
            is_jammed: true,
        };

        assert!(jammed_result.is_effectively_jammed());
        assert!(jammed_result.quality_score() < 0.1);
    }
}
