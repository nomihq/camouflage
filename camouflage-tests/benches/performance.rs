//! Performance benchmarks for camouflage signal generation
//!
//! These benchmarks measure:
//! - Signal generation performance across different tone configurations
//! - Impact of sample rate on performance
//! - Buffer size effects on throughput
//!
//! Run with: cargo bench

use camouflage_core::{SignalConfig, SignalGenerator};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

/// Benchmark signal generation with varying number of tones
fn benchmark_signal_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_generation");

    for num_tones in [1, 3, 5, 7] {
        group.bench_with_input(
            BenchmarkId::new("tones", num_tones),
            &num_tones,
            |b, &num_tones| {
                let config = SignalConfig {
                    num_tones,
                    ..Default::default()
                };
                let mut generator = SignalGenerator::new(config);

                b.iter(|| {
                    let mut buffer = [0.0f32; 1024];
                    generator.generate_buffer(black_box(&mut buffer));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark impact of different sample rates on performance
fn benchmark_sample_rates(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample_rates");

    for sample_rate in [44100, 48000, 96000] {
        group.bench_with_input(
            BenchmarkId::new("rate", sample_rate),
            &sample_rate,
            |b, &sample_rate| {
                let config = SignalConfig {
                    sample_rate,
                    ..Default::default()
                };
                let mut generator = SignalGenerator::new(config);

                b.iter(|| {
                    let mut buffer = [0.0f32; 1024];
                    generator.generate_buffer(black_box(&mut buffer));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark throughput with different buffer sizes
fn benchmark_buffer_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_sizes");

    for buffer_size in [256, 512, 1024, 2048, 4096] {
        group.bench_with_input(
            BenchmarkId::new("size", buffer_size),
            &buffer_size,
            |b, &buffer_size| {
                let config = SignalConfig::default();
                let mut generator = SignalGenerator::new(config);

                b.iter(|| {
                    let mut buffer = vec![0.0f32; buffer_size];
                    generator.generate_buffer(black_box(&mut buffer));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_signal_generation,
    benchmark_sample_rates,
    benchmark_buffer_sizes
);
criterion_main!(benches);
