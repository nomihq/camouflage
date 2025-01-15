use camouflage_core::SignalConfig;
use camouflage_tests::{OpenAITTS, WhisperClient};
use std::env;
use tempfile::TempDir;
use tracing::{info, warn};
use tracing_subscriber;

const TEST_PHRASE: &str = "The quick brown fox jumps over the lazy dog.";

#[tokio::test]
#[ignore] // Run with: cargo test --test e2e_whisper -- --ignored
async fn test_whisper_transcribes_clean_audio() {
    let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

    info!("=== E2E Whisper Clean Audio Test ===");

    let api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not set. Please set it to run E2E tests.");

    let tts = OpenAITTS::new(api_key.clone());
    let whisper = WhisperClient::new(api_key);

    let temp_dir = TempDir::new().unwrap();
    let audio_path = temp_dir.path().join("clean_voice.flac");

    info!("Step 1: Generate clean voice sample with OpenAI TTS");
    tts.generate_speech(TEST_PHRASE, &audio_path, None)
        .await
        .expect("Failed to generate TTS");

    info!("Step 2: Transcribe with OpenAI Whisper");
    let result = whisper
        .transcribe_file(&audio_path)
        .await
        .expect("Failed to transcribe");

    info!("Transcription result:");
    info!("  Original: '{}'", TEST_PHRASE);
    info!("  Transcribed: '{}'", result.transcript);
    info!("  Word count: {}", result.word_count);

    // Assert clean audio transcribes well
    assert!(
        result.word_count >= 9,
        "Should transcribe at least 9 words, got: {}",
        result.word_count
    );
    assert!(
        !result.is_effectively_jammed(),
        "Clean audio should not be jammed"
    );

    info!("✓ Clean audio transcribes successfully with Whisper");
}

#[tokio::test]
#[ignore]
async fn test_whisper_pure_ultrasonic_not_transcribable() {
    let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

    info!("=== Pure Ultrasonic Whisper Test ===");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let whisper = WhisperClient::new(api_key);

    let temp_dir = TempDir::new().unwrap();
    let ultrasonic_path = temp_dir.path().join("pure_ultrasonic.wav");

    info!("Step 1: Generating pure ultrasonic audio (23kHz, 3 seconds)");
    camouflage_tests::test_utils::generate_pure_ultrasonic(
        &ultrasonic_path,
        3.0,
        &SignalConfig::default(),
    )
    .expect("Failed to generate ultrasonic audio");

    info!("Step 2: Attempting to transcribe with Whisper");
    let result = whisper
        .transcribe_file(&ultrasonic_path)
        .await
        .expect("Failed to transcribe");

    info!("Pure ultrasonic transcription result:");
    info!("  Transcript: '{}'", result.transcript);
    info!("  Word count: {}", result.word_count);
    info!("  Effectively jammed: {}", result.is_effectively_jammed());

    // Pure ultrasonic should not produce meaningful transcription
    assert!(
        result.is_effectively_jammed(),
        "Pure ultrasonic should not transcribe. Got {} words: '{}'",
        result.word_count,
        result.transcript
    );

    info!("✓ Pure ultrasonic audio is not transcribable by Whisper");
    info!("✓ Camouflage disrupts OpenAI Whisper speech recognition");
}

#[tokio::test]
#[ignore]
async fn test_whisper_multiple_configurations() {
    let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

    info!("=== Whisper Multiple Configurations Test ===");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let whisper = WhisperClient::new(api_key);
    let temp_dir = TempDir::new().unwrap();

    let configurations = vec![
        ("single_tone", SignalConfig {
            frequency: 23000.0,
            sample_rate: 44100,
            amplitude: 0.3,
            num_tones: 1,
            frequency_spread: 0.0,
        }),
        ("multi_tone_3", SignalConfig::default()),
        ("multi_tone_5", SignalConfig {
            frequency: 22000.0,
            sample_rate: 44100,
            amplitude: 0.3,
            num_tones: 5,
            frequency_spread: 400.0,
        }),
    ];

    info!("Testing various ultrasonic configurations against Whisper...\n");

    for (name, config) in &configurations {
        info!("--- Configuration: {} ---", name);
        info!("  Frequency: {} Hz", config.frequency);
        info!("  Tones: {}", config.num_tones);

        let audio_path = temp_dir.path().join(format!("{}.wav", name));

        camouflage_tests::test_utils::generate_pure_ultrasonic(&audio_path, 2.0, config)
            .expect("Failed to generate signal");

        let result = whisper
            .transcribe_file(&audio_path)
            .await
            .expect("Transcription failed");

        info!("  Transcript: '{}'", result.transcript);
        info!("  Word count: {}", result.word_count);
        info!("  Jammed: {}\n", result.is_effectively_jammed());

        assert!(
            result.is_effectively_jammed(),
            "Configuration {} should produce untranscribable audio",
            name
        );
    }

    info!("✓ All ultrasonic configurations disrupt Whisper");
}

#[tokio::test]
#[ignore]
async fn test_whisper_vs_deepgram_comparison() {
    let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

    info!("=== Whisper vs Deepgram Comparison ===");

    let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let deepgram_key = env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY not set");

    let whisper = WhisperClient::new(openai_key);
    let deepgram = camouflage_tests::DeepgramClient::new(deepgram_key);

    let temp_dir = TempDir::new().unwrap();
    let ultrasonic_path = temp_dir.path().join("ultrasonic.wav");

    info!("Generating ultrasonic audio...");
    camouflage_tests::test_utils::generate_pure_ultrasonic(
        &ultrasonic_path,
        3.0,
        &SignalConfig::default(),
    )
    .expect("Failed to generate");

    info!("\nTranscribing with both Whisper and Deepgram...");

    let whisper_result = whisper
        .transcribe_file(&ultrasonic_path)
        .await
        .expect("Whisper failed");

    let deepgram_result = deepgram
        .transcribe_file(&ultrasonic_path)
        .await
        .expect("Deepgram failed");

    info!("\n=== Results ===");
    info!("Whisper:");
    info!("  Transcript: '{}'", whisper_result.transcript);
    info!("  Word count: {}", whisper_result.word_count);
    info!("  Jammed: {}", whisper_result.is_effectively_jammed());

    info!("\nDeepgram:");
    info!("  Transcript: '{}'", deepgram_result.transcript);
    info!("  Confidence: {:.2}", deepgram_result.confidence);
    info!("  Word count: {}", deepgram_result.word_count);
    info!("  Jammed: {}", deepgram_result.is_effectively_jammed());

    // Both should be jammed
    assert!(
        whisper_result.is_effectively_jammed(),
        "Whisper should be jammed"
    );
    assert!(
        deepgram_result.is_effectively_jammed(),
        "Deepgram should be jammed"
    );

    info!("\n✓ Camouflage defeats both Whisper and Deepgram");
}
