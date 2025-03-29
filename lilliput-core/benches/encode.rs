use std::{
    convert::Infallible,
    time::{Duration, Instant},
};

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng, SeedableRng,
};
use rand_xorshift::XorShiftRng;

use lilliput_core::{
    encoder::{Encoder, EncoderConfig},
    error::Result,
    io::Write,
};

struct PlaceboWriter;

impl Write for PlaceboWriter {
    type Error = Infallible;

    #[inline(never)]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        black_box(Ok(buf.len()))
    }

    #[inline(never)]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

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

fn bench_sampled<T, I, Fi, F>(
    bencher: &mut Bencher<'_>,
    config: EncoderConfig,
    values: Fi,
    mut f: F,
) where
    T: Copy,
    Fi: Fn() -> I,
    I: Iterator<Item = T>,
    F: FnMut(&mut Encoder<PlaceboWriter>, T),
{
    bencher.iter_custom(|iters| {
        let mut total_duration = Duration::ZERO;

        for _ in 0..iters {
            let mut samples: u32 = 0;

            let writer = PlaceboWriter;
            let mut encoder = Encoder::new(writer, config);

            // The actual operation we're measuring is faster than we can reasonably measure
            // without significant bias from the overhead of computing `Instant`/`Duration`,
            // so rather than measure a single operation we measure a batch of operations,
            // based on randomized input bytes:
            let start = Instant::now();
            for value in values() {
                f(black_box(&mut encoder), black_box(value));
                samples += 1;
            }
            let duration = start.elapsed();

            // Calculate mean duration over sampled values:
            total_duration += duration.checked_div(samples).unwrap();
        }

        total_duration
    })
}

fn bench_int(c: &mut Criterion, label: &str, config: EncoderConfig) {
    fn values<T>() -> impl Iterator<Item = T>
    where
        StandardUniform: Distribution<T>,
    {
        sampling_values_iter(SAMPLES)
    }

    let mut g = c.benchmark_group("int");

    g.bench_function(format!("encode_u8 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_u8(black_box(value))).unwrap();
        });
    });

    g.bench_function(format!("encode_i8 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_i8(black_box(value))).unwrap();
        });
    });

    g.bench_function(format!("encode_u16 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_u16(black_box(value))).unwrap();
        });
    });

    g.bench_function(format!("encode_i16 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_i16(black_box(value))).unwrap();
        });
    });

    g.bench_function(format!("encode_u32 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_u32(black_box(value))).unwrap();
        });
    });

    g.bench_function(format!("encode_i32 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_i32(black_box(value))).unwrap();
        });
    });

    g.bench_function(format!("encode_u64 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_u64(black_box(value))).unwrap();
        });
    });

    g.bench_function(format!("encode_i64 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_i64(black_box(value))).unwrap();
        });
    });

    g.finish();
}

fn bench_string(c: &mut Criterion, label: &str, config: EncoderConfig) {
    let mut g = c.benchmark_group("string");

    fn lengths() -> impl Iterator<Item = usize>
    where
        StandardUniform: Distribution<u32>,
    {
        sampling_values_iter::<u32>(SAMPLES).map(|len| len as usize)
    }

    g.bench_function(format!("encode_str_header @ {label}"), |b| {
        bench_sampled(b, config, lengths, |encoder, len| {
            black_box(encoder.encode_str_header(black_box(len))).unwrap();
        });
    });

    g.finish();
}

fn bench_seq(c: &mut Criterion, label: &str, config: EncoderConfig) {
    let mut g = c.benchmark_group("seq");

    fn lengths() -> impl Iterator<Item = usize>
    where
        StandardUniform: Distribution<u32>,
    {
        sampling_values_iter::<u32>(SAMPLES).map(|len| len as usize)
    }

    g.bench_function(format!("encode_seq_header @ {label}"), |b| {
        bench_sampled(b, config, lengths, |encoder, len| {
            black_box(encoder.encode_seq_header(black_box(len))).unwrap();
        });
    });

    g.finish();
}

fn bench_map(c: &mut Criterion, label: &str, config: EncoderConfig) {
    let mut g = c.benchmark_group("map");

    fn lengths() -> impl Iterator<Item = usize>
    where
        StandardUniform: Distribution<u32>,
    {
        sampling_values_iter::<u32>(SAMPLES).map(|len| len as usize)
    }

    g.bench_function(format!("encode_map_header @ {label}"), |b| {
        bench_sampled(b, config, lengths, |encoder, len| {
            black_box(encoder.encode_map_header(black_box(len))).unwrap();
        });
    });

    g.finish();
}

fn bench_float(c: &mut Criterion, label: &str, config: EncoderConfig) {
    let mut g = c.benchmark_group("float");

    fn values<T>() -> impl Iterator<Item = T>
    where
        StandardUniform: Distribution<T>,
    {
        sampling_values_iter(SAMPLES)
    }

    g.bench_function(format!("encode_f32 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_f32(value)).unwrap();
        });
    });

    g.bench_function(format!("encode_f64 @ {label}"), |b| {
        bench_sampled(b, config, values, |encoder, value| {
            black_box(encoder.encode_f64(value)).unwrap();
        });
    });

    g.finish();
}

fn bench_bytes(c: &mut Criterion, label: &str, config: EncoderConfig) {
    fn lengths() -> impl Iterator<Item = usize>
    where
        StandardUniform: Distribution<u32>,
    {
        sampling_values_iter::<u32>(SAMPLES).map(|len| len as usize)
    }

    let mut g = c.benchmark_group("bytes");

    g.bench_function(format!("encode_bytes_header @ {label}"), |b| {
        bench_sampled(b, config, lengths, |encoder, len| {
            black_box(encoder.encode_bytes_header(black_box(len))).unwrap();
        });
    });

    g.finish();
}

fn bench_bool(c: &mut Criterion, label: &str, config: EncoderConfig) {
    fn flags() -> impl Iterator<Item = bool>
    where
        StandardUniform: Distribution<bool>,
    {
        sampling_values_iter(SAMPLES)
    }

    let mut g = c.benchmark_group("bool");

    g.bench_function(format!("encode_bool @ {label}"), |b| {
        bench_sampled(b, config, flags, |encoder, flag| {
            black_box(encoder.encode_bool(black_box(flag))).unwrap();
        });
    });

    g.finish();
}

fn bench_null(c: &mut Criterion, label: &str, config: EncoderConfig) {
    fn units() -> impl Iterator<Item = ()> {
        std::iter::repeat_n((), SAMPLES)
    }

    let mut g = c.benchmark_group("null");

    g.bench_function(format!("encode_null @ {label}"), |b| {
        bench_sampled(b, config, units, |encoder, ()| {
            black_box(encoder.encode_null()).unwrap();
        });
    });

    g.finish();
}

fn benchmark_with_config(c: &mut Criterion, label: &str, config: EncoderConfig) {
    bench_int(c, label, config);
    bench_string(c, label, config);
    bench_seq(c, label, config);
    bench_map(c, label, config);
    bench_float(c, label, config);
    bench_bytes(c, label, config);
    bench_bool(c, label, config);
    bench_null(c, label, config);
}

fn benchmark_default_config(c: &mut Criterion) {
    benchmark_with_config(c, "default", EncoderConfig::default());
}

criterion_group!(default_config, benchmark_default_config);

criterion_main!(default_config);
