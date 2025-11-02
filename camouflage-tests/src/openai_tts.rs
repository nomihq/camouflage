use anyhow::{Context, Result};
use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use tracing::info;

const TTS_API_URL: &str = "https://api.openai.com/v1/audio/speech";

#[derive(Debug, Serialize)]
struct TTSRequest {
    model: String,
    input: String,
    voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<String>,
}

pub struct OpenAITTS {
    client: Client,
    api_key: String,
}

impl OpenAITTS {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Generate speech from text using OpenAI TTS
    pub async fn generate_speech(
        &self,
        text: &str,
        output_path: &Path,
        voice: Option<&str>,
    ) -> Result<()> {
        info!("Generating speech with OpenAI TTS");
        info!("  Text: {}", text);

        let request = TTSRequest {
            model: "tts-1".to_string(),
            input: text.to_string(),
            voice: voice.unwrap_or("alloy").to_string(),
            response_format: Some("flac".to_string()),
        };

        let response = self
            .client
            .post(TTS_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .context("Failed to send TTS request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("OpenAI TTS API error ({}): {}", status, error_text);
        }

        let audio_data = response.bytes().await?;

        let mut file = tokio::fs::File::create(output_path).await?;
        file.write_all(&audio_data).await?;

        info!("  Saved to: {}", output_path.display());
        Ok(())
    }
}
