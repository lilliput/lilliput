use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use lilliput_float::{FpPack as _, PackedFloatValidator, F32, F64};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng, SeedableRng,
};
use rand_xorshift::XorShiftRng;

const CRITERION_SIGNIFICANCE_LEVEL: f64 = 0.1;
const CRITERION_SAMPLE_SIZE: usize = 500;

const SAMPLES: usize = 65_536;
const RNG_SEED: u64 = 42;

fn seeded_rng() -> XorShiftRng {
    XorShiftRng::seed_from_u64(RNG_SEED)
}

fn sampling_values_iter<T>(samples: usize) -> impl Iterator<Item = T>
where
    StandardUniform: Distribution<T>,
{
    seeded_rng().random_iter().take(samples)
}

fn bench_truncate_f32_with_samples(
    g: &mut BenchmarkGroup<'_, WallTime>,
    label: &str,
    values: &[f32],
    validator: &PackedFloatValidator<f32>,
) {
    let values_len = values.len();

    g.bench_function(label, |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            for _ in 0..iters {
                let start = Instant::now();

                for native_value in values {
                    let value = F32::from(*native_value);
                    black_box(black_box(value).pack_optimal(validator));
                }

                // Calculate mean duration over the sampled headers:
                duration += start.elapsed().checked_div(values_len as u32).unwrap();
            }

            duration
        });
    });
}

fn bench_truncate_f64_with_samples(
    g: &mut BenchmarkGroup<'_, WallTime>,
    label: &str,
    values: &[f64],
    validator: &PackedFloatValidator<f64>,
) {
    let values_len = values.len();

    g.bench_function(label, |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            for _ in 0..iters {
                let start = Instant::now();

                for native_value in values {
                    let value = F64::from(*native_value);
                    black_box(black_box(value).pack_optimal(validator));
                }

                // Calculate mean duration over the sampled headers:
                duration += start.elapsed().checked_div(values_len as u32).unwrap();
            }

            duration
        });
    });
}

fn bench_f32_truncate(c: &mut Criterion) {
    let mut g = c.benchmark_group("float");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<f32> = sampling_values_iter::<f32>(SAMPLES).collect();
    let validator = PackedFloatValidator::Relative(0.0001);
    bench_truncate_f32_with_samples(&mut g, "pack f32", &samples, &validator);

    g.finish();
}

fn bench_f64_truncate(c: &mut Criterion) {
    let mut g = c.benchmark_group("float");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<f64> = sampling_values_iter::<f64>(SAMPLES).collect();
    let validator = PackedFloatValidator::Relative(0.0001);
    bench_truncate_f64_with_samples(&mut g, "pack f64", &samples, &validator);

    g.finish();
}

fn benchmark_with_config(c: &mut Criterion) {
    bench_f32_truncate(c);
    bench_f64_truncate(c);
}

fn benchmark_default_config(c: &mut Criterion) {
    benchmark_with_config(c);
}

criterion_group!(default_config, benchmark_default_config);

criterion_main!(default_config);
