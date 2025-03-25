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

use lilliput_core::{encoder::Encoder, io::Write};

const LENGTHS: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

struct PlaceboWriter;

impl Write for PlaceboWriter {
    type Error = Infallible;

    #[inline(never)]
    fn write(&mut self, buf: &[u8]) -> lilliput_core::error::Result<usize> {
        black_box(Ok(buf.len()))
    }

    #[inline(never)]
    fn flush(&mut self) -> lilliput_core::error::Result<()> {
        Ok(())
    }
}

const RNG_SEED: u64 = 42;

fn placebo_encoder() -> Encoder<PlaceboWriter> {
    Encoder::new(PlaceboWriter).compact_ints()
}

fn sampling_values_iter<T>(samples: usize) -> impl Iterator<Item = T>
where
    StandardUniform: Distribution<T>,
{
    XorShiftRng::seed_from_u64(RNG_SEED)
        .random_iter()
        .take(samples)
}

fn bench_sampled<T, W, I, Fe, Fi, F>(bencher: &mut Bencher<'_>, encoder: Fe, values: Fi, mut f: F)
where
    T: Copy,
    W: Write,
    Fe: Fn() -> Encoder<W>,
    Fi: Fn() -> I,
    I: Iterator<Item = T>,
    F: FnMut(&mut Encoder<W>, T),
{
    bencher.iter_custom(|iters| {
        let mut total_duration = Duration::ZERO;

        for _ in 0..iters {
            let mut iter_duration = Duration::ZERO;

            let mut samples: u32 = 0;
            let mut encoder = encoder();

            let start = Instant::now();
            for value in values() {
                f(black_box(&mut encoder), black_box(value));
                samples += 1;
            }
            iter_duration += start.elapsed();

            // Calculate mean duration over sampled values:
            total_duration += iter_duration.checked_div(samples).unwrap();
        }

        total_duration
    })
}

fn bench_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("int");

    const SAMPLES: usize = 65_536;

    let encoder = placebo_encoder;

    fn values<T>() -> impl Iterator<Item = T>
    where
        StandardUniform: Distribution<T>,
    {
        sampling_values_iter(SAMPLES)
    }

    group.bench_function("u8", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_u8(value).unwrap();
        });
    });

    group.bench_function("i8", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_i8(value).unwrap();
        });
    });

    group.bench_function("u16", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_u16(value).unwrap();
        });
    });

    group.bench_function("i16", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_i16(value).unwrap();
        });
    });

    group.bench_function("u32", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_u32(value).unwrap();
        });
    });

    group.bench_function("i32", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_i32(value).unwrap();
        });
    });

    group.bench_function("u64", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_u64(value).unwrap();
        });
    });

    group.bench_function("i64", |b| {
        bench_sampled(b, encoder, values, |encoder, value| {
            encoder.encode_i64(value).unwrap();
        });
    });

    group.finish();
}

fn bench_string(c: &mut Criterion) {
    const SAMPLES: usize = 256;

    let mut group = c.benchmark_group("string");

    group.bench_function("head-only", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let lengths: &[usize] = LENGTHS;

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                for &len in lengths {
                    encoder.encode_str_start(black_box(len)).unwrap();
                }
                duration += start.elapsed();
            }

            // Calculate mean duration over inputs:
            duration.checked_div(SAMPLES as u32).unwrap()
        })
    });

    group.bench_function("full", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let inputs: Vec<String> = LENGTHS
                .into_iter()
                .map(|&len| {
                    (1..=len)
                        .map(|i| char::from_u32(i as u32).unwrap())
                        .collect()
                })
                .collect();

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                for input in &inputs {
                    encoder.encode_str(black_box(&input)).unwrap();
                }
                duration += start.elapsed();
            }

            // Calculate mean duration over inputs:
            duration.checked_div(SAMPLES as u32).unwrap()
        })
    });
}

fn bench_seq(c: &mut Criterion) {
    const SAMPLES: usize = 65_536;

    let mut group = c.benchmark_group("seq");

    group.bench_function("head-only", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let lengths: &[usize] = LENGTHS;

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                for &len in lengths {
                    encoder.encode_seq_start(black_box(len)).unwrap();
                }
                duration += start.elapsed();
            }

            // Calculate mean duration over inputs:
            duration.checked_div(SAMPLES as u32).unwrap()
        })
    });
}

fn bench_map(c: &mut Criterion) {
    const SAMPLES: usize = 65_536;

    let mut group = c.benchmark_group("map");

    group.bench_function("head-only", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let lengths = LENGTHS;

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                for &len in lengths {
                    encoder.encode_map_start(black_box(len)).unwrap();
                }
                duration += start.elapsed();
            }

            // Calculate mean duration over inputs:
            duration.checked_div(SAMPLES as u32).unwrap()
        })
    });
}

fn bench_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("float");

    const SAMPLES: usize = 65_536;

    group.bench_function("f32", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let samples: u32 = SAMPLES as u32;

            let inputs: Vec<f32> = (0..samples)
                .map(|sample| f32::from_bits((u32::MAX / samples) * sample))
                .collect();

            for input in inputs {
                for _ in 0..iters {
                    let mut encoder = placebo_encoder();

                    let start = Instant::now();
                    encoder.encode_f32(black_box(input)).unwrap();
                    duration += start.elapsed();
                }
            }

            // Calculate mean duration over inputs:
            duration.checked_div(samples as u32).unwrap()
        })
    });

    group.bench_function("f64", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let samples: u64 = SAMPLES as u64;

            let inputs: Vec<f64> = (0..samples)
                .map(|sample| f64::from_bits((u64::MAX / samples) * sample))
                .collect();

            for input in inputs {
                for _ in 0..iters {
                    let mut encoder = placebo_encoder();

                    let start = Instant::now();
                    encoder.encode_f64(black_box(input)).unwrap();
                    duration += start.elapsed();
                }
            }

            // Calculate mean duration over inputs:
            duration.checked_div(samples as u32).unwrap()
        })
    });

    group.finish();
}

fn bench_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes");

    group.bench_function("head-only", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let lengths: &[usize] = LENGTHS;

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                for &len in lengths {
                    encoder.encode_bytes_start(black_box(len)).unwrap();
                }
                duration += start.elapsed();
            }

            // Calculate mean duration over inputs:
            duration.checked_div(lengths.len() as u32).unwrap()
        })
    });

    group.bench_function("full", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let lengths = LENGTHS;
            let inputs: Vec<Vec<u8>> = lengths.into_iter().map(|&len| vec![0; len]).collect();

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                for input in &inputs {
                    encoder.encode_bytes(black_box(&input)).unwrap();
                }
                duration += start.elapsed();
            }

            // Calculate mean duration over inputs:
            duration.checked_div(lengths.len() as u32).unwrap()
        })
    });
}

fn bench_bool(c: &mut Criterion) {
    const SAMPLES: usize = 2;

    c.bench_function("bool", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            let inputs: Vec<bool> = (0..SAMPLES).map(|i| i % 2 == 0).collect();

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                for &input in &inputs {
                    encoder.encode_bool(black_box(input)).unwrap();
                }
                duration += start.elapsed();
            }

            // Calculate mean duration over inputs:
            duration.checked_div(SAMPLES as u32).unwrap()
        })
    });
}

fn bench_null(c: &mut Criterion) {
    c.bench_function("null", |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            for _ in 0..iters {
                let mut encoder = placebo_encoder();

                let start = Instant::now();
                encoder.encode_null().unwrap();
                duration += start.elapsed();
            }

            duration
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_int(c);
    bench_string(c);
    bench_seq(c);
    bench_map(c);
    bench_float(c);
    bench_bytes(c);
    bench_bool(c);
    bench_null(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
