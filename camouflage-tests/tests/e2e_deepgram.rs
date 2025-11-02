use camouflage_core::SignalConfig;
use camouflage_tests::{DeepgramClient, OpenAITTS};
use std::env;
use tempfile::TempDir;
use tracing::info;
use tracing_subscriber;

const TEST_PHRASE: &str = "The quick brown fox jumps over the lazy dog.";

#[tokio::test]
#[ignore] // Run with: cargo test --test e2e_deepgram -- --ignored
async fn test_deepgram_transcribes_clean_audio() {
    let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

    info!("=== E2E Deepgram Clean Audio Test ===");

    let openai_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not set. Please set it to run E2E tests.");
    let deepgram_key = env::var("DEEPGRAM_API_KEY")
        .expect("DEEPGRAM_API_KEY not set. Please set it to run E2E tests.");

    let tts = OpenAITTS::new(openai_key);
    let deepgram = DeepgramClient::new(deepgram_key);

    let temp_dir = TempDir::new().unwrap();
    let audio_path = temp_dir.path().join("clean_voice.flac");

    info!("Step 1: Generate clean voice sample with OpenAI TTS");
    tts.generate_speech(TEST_PHRASE, &audio_path, None)
        .await
        .expect("Failed to generate TTS");

    info!("Step 2: Transcribe with Deepgram");
    let result = deepgram
        .transcribe_file(&audio_path)
        .await
        .expect("Failed to transcribe");

    info!("Transcription result:");
    info!("  Original: '{}'", TEST_PHRASE);
    info!("  Transcribed: '{}'", result.transcript);
    info!("  Confidence: {:.2}", result.confidence);
    info!("  Word count: {}", result.word_count);

    // Assert clean audio transcribes well
    assert!(
        result.confidence > 0.7,
        "Clean audio should transcribe with high confidence, got: {}",
        result.confidence
    );
    assert!(
        result.word_count >= 9,
        "Should transcribe at least 9 words, got: {}",
        result.word_count
    );

    info!("✓ Clean audio transcribes successfully with Deepgram");
}

#[tokio::test]
#[ignore]
async fn test_deepgram_pure_ultrasonic_not_transcribable() {
    let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

    info!("=== Pure Ultrasonic Deepgram Test ===");

    let deepgram_key = env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY not set");
    let deepgram = DeepgramClient::new(deepgram_key);

    let temp_dir = TempDir::new().unwrap();
    let ultrasonic_path = temp_dir.path().join("pure_ultrasonic.wav");

    info!("Step 1: Generating pure ultrasonic audio (23kHz, 3 seconds)");
    camouflage_tests::test_utils::generate_pure_ultrasonic(
        &ultrasonic_path,
        3.0,
        &SignalConfig::default(),
    )
    .expect("Failed to generate ultrasonic audio");

    info!("Step 2: Attempting to transcribe with Deepgram");
    let result = deepgram
        .transcribe_file(&ultrasonic_path)
        .await
        .expect("Failed to transcribe");

    info!("Pure ultrasonic transcription result:");
    info!("  Transcript: '{}'", result.transcript);
    info!("  Confidence: {:.2}", result.confidence);
    info!("  Word count: {}", result.word_count);
    info!("  Effectively jammed: {}", result.is_effectively_jammed());

    // Pure ultrasonic should not produce meaningful transcription
    assert!(
        result.is_effectively_jammed(),
        "Pure ultrasonic should not transcribe. Got {} words with {:.2} confidence: '{}'",
        result.word_count,
        result.confidence,
        result.transcript
    );

    info!("✓ Pure ultrasonic audio is not transcribable by Deepgram");
    info!("✓ Camouflage disrupts Deepgram speech recognition");
}

#[tokio::test]
#[ignore]
async fn test_deepgram_multiple_configurations() {
    let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

    info!("=== Deepgram Multiple Configurations Test ===");

    let deepgram_key = env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY not set");
    let deepgram = DeepgramClient::new(deepgram_key);
    let temp_dir = TempDir::new().unwrap();

    let configurations = vec![
        (
            "single_tone",
            SignalConfig {
                frequency: 23000.0,
                sample_rate: 44100,
                amplitude: 0.3,
                num_tones: 1,
                frequency_spread: 0.0,
            },
        ),
        ("multi_tone_3", SignalConfig::default()),
        (
            "multi_tone_5",
            SignalConfig {
                frequency: 22000.0,
                sample_rate: 44100,
                amplitude: 0.3,
                num_tones: 5,
                frequency_spread: 400.0,
            },
        ),
    ];

    info!("Testing various ultrasonic configurations against Deepgram...\n");

    for (name, config) in &configurations {
        info!("--- Configuration: {} ---", name);
        info!("  Frequency: {} Hz", config.frequency);
        info!("  Tones: {}", config.num_tones);

        let audio_path = temp_dir.path().join(format!("{}.wav", name));

        camouflage_tests::test_utils::generate_pure_ultrasonic(&audio_path, 2.0, config)
            .expect("Failed to generate signal");

        let result = deepgram
            .transcribe_file(&audio_path)
            .await
            .expect("Transcription failed");

        info!("  Transcript: '{}'", result.transcript);
        info!("  Confidence: {:.2}", result.confidence);
        info!("  Word count: {}", result.word_count);
        info!("  Jammed: {}\n", result.is_effectively_jammed());

        assert!(
            result.is_effectively_jammed(),
            "Configuration {} should produce untranscribable audio",
            name
        );
    }

    info!("✓ All ultrasonic configurations disrupt Deepgram");
}
