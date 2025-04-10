use std::time::{Duration, Instant};

use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng, SeedableRng,
};
use rand_xorshift::XorShiftRng;

use lilliput_core::{
    config::{EncodingConfig, PackingMode},
    decoder::Decoder,
    encoder::Encoder,
    header::{
        BoolHeader, BytesHeader, FloatHeader, Header, IntHeader, MapHeader, NullHeader, SeqHeader,
        StringHeader, UnitHeader,
    },
    io::{SliceReader, VecWriter},
    value::IntValue,
};

const CRITERION_SIGNIFICANCE_LEVEL: f64 = 0.1;
const CRITERION_SAMPLE_SIZE: usize = 500;

// Value headers have a size between 1 and 9 bytes:
const WIRE_SIZE_HINT: usize = 10;
const SAMPLES: usize = 65_536;
const CAPACITY: usize = SAMPLES * WIRE_SIZE_HINT;

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

fn bench_roundtrip_with_samples(
    g: &mut BenchmarkGroup<'_, WallTime>,
    label: Option<&str>,
    headers: &[Header],
    config: EncodingConfig,
) {
    let headers_len = headers.len();

    let mut scratch = Vec::with_capacity(CAPACITY);

    let encode_id = if let Some(label) = label {
        format!("encode {label}")
    } else {
        "encode".to_owned()
    };

    g.bench_function(encode_id, |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            for _ in 0..iters {
                scratch.clear();

                let writer = VecWriter::new(&mut scratch);
                let mut encoder = Encoder::new(writer, config);

                let start = Instant::now();

                for header in headers {
                    let _ = black_box(encoder.encode_header(black_box(header)));
                }

                // Calculate mean duration over the sampled headers:
                duration += start.elapsed().checked_div(headers_len as u32).unwrap();
            }

            duration
        });
    });

    assert!(
        scratch.len() <= CAPACITY,
        "resize detected, scratch buffer capacity should probably be increased"
    );

    let encoded: Vec<u8> = {
        let mut buf = Vec::with_capacity(CAPACITY);

        let writer = VecWriter::new(&mut buf);
        let mut encoder = Encoder::new(writer, config);

        for header in headers {
            let _ = encoder.encode_header(header);
        }

        buf
    };

    let decode_id = if let Some(label) = label {
        format!("decode {label}")
    } else {
        "decode".to_owned()
    };

    g.bench_function(decode_id, |b| {
        b.iter_custom(|iters| {
            let mut duration = Duration::ZERO;

            for _ in 0..iters {
                let reader = SliceReader::new(&encoded);
                let mut decoder = Decoder::new(reader);

                let start = Instant::now();

                for _ in 0..headers_len {
                    let _ = black_box(decoder.decode_header());
                }

                // Calculate mean duration over sampled values:
                duration += start.elapsed().checked_div(headers_len as u32).unwrap();
            }

            duration
        });
    });
}

fn bench_int(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter<T>(samples: usize) -> impl Iterator<Item = Header>
    where
        StandardUniform: Distribution<T>,
        IntValue: From<T>,
    {
        sampling_values_iter::<u8>(samples).map(move |random_bits| {
            let is_compact: bool = random_bits & 0b01000000 != 0b0;
            let is_signed: bool = random_bits & 0b00100000 != 0b0;
            if is_compact {
                let bits: u8 = random_bits & 0b00000111;
                Header::Int(IntHeader::compact(is_signed, bits))
            } else {
                let width: u8 = 1 + (random_bits & 0b00000111);
                Header::Int(IntHeader::extended(is_signed, width))
            }
        })
    }

    let mut g = c.benchmark_group("string");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter::<u8>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("u8"), &samples, config);

    let samples: Vec<Header> = samples_iter::<u16>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("u16"), &samples, config);

    let samples: Vec<Header> = samples_iter::<u32>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("u32"), &samples, config);

    let samples: Vec<Header> = samples_iter::<u64>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("u64"), &samples, config);

    let samples: Vec<Header> = samples_iter::<i8>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("i8"), &samples, config);

    let samples: Vec<Header> = samples_iter::<i16>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("i16"), &samples, config);

    let samples: Vec<Header> = samples_iter::<i32>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("i32"), &samples, config);

    let samples: Vec<Header> = samples_iter::<i64>(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, Some("i64"), &samples, config);

    g.finish();
}

fn bench_string(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize, packing_mode: PackingMode) -> impl Iterator<Item = Header> {
        sampling_values_iter::<u32>(samples)
            .map(move |len| Header::String(StringHeader::for_len(len as usize, packing_mode)))
    }

    let mut g = c.benchmark_group("string");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES, config.len_packing).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn bench_seq(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize, packing_mode: PackingMode) -> impl Iterator<Item = Header> {
        sampling_values_iter::<u32>(samples)
            .map(move |len| Header::Seq(SeqHeader::for_len(len as usize, packing_mode)))
    }

    let mut g = c.benchmark_group("seq");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES, config.len_packing).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn bench_map(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize, packing_mode: PackingMode) -> impl Iterator<Item = Header> {
        sampling_values_iter::<u32>(samples)
            .map(move |len| Header::Map(MapHeader::for_len(len as usize, packing_mode)))
    }

    let mut g = c.benchmark_group("map");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES, config.len_packing).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn bench_float(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize) -> impl Iterator<Item = Header> {
        sampling_values_iter::<u8>(samples)
            .map(move |width| Header::Float(FloatHeader::new((width % 8) + 1)))
    }

    let mut g = c.benchmark_group("float");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn bench_bytes(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize) -> impl Iterator<Item = Header> {
        sampling_values_iter::<u32>(samples)
            .map(move |len| Header::Bytes(BytesHeader::for_len(len as usize)))
    }

    let mut g = c.benchmark_group("bytes");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn bench_bool(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize) -> impl Iterator<Item = Header> {
        sampling_values_iter::<bool>(samples).map(move |value| Header::Bool(BoolHeader::new(value)))
    }

    let mut g = c.benchmark_group("bool");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn bench_unit(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize) -> impl Iterator<Item = Header> {
        std::iter::repeat_n(Header::Unit(UnitHeader), samples)
    }

    let mut g = c.benchmark_group("null");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn bench_null(c: &mut Criterion, config: EncodingConfig) {
    fn samples_iter(samples: usize) -> impl Iterator<Item = Header> {
        std::iter::repeat_n(Header::Null(NullHeader), samples)
    }

    let mut g = c.benchmark_group("null");

    g.significance_level(CRITERION_SIGNIFICANCE_LEVEL);
    g.sample_size(CRITERION_SAMPLE_SIZE);

    let samples: Vec<Header> = samples_iter(SAMPLES).collect();
    bench_roundtrip_with_samples(&mut g, None, &samples, config);

    g.finish();
}

fn benchmark_with_config(c: &mut Criterion, config: EncodingConfig) {
    bench_int(c, config);
    bench_string(c, config);
    bench_seq(c, config);
    bench_map(c, config);
    bench_float(c, config);
    bench_bytes(c, config);
    bench_bool(c, config);
    bench_unit(c, config);
    bench_null(c, config);
}

fn benchmark_default_config(c: &mut Criterion) {
    benchmark_with_config(c, EncodingConfig::default());
}

criterion_group!(default_config, benchmark_default_config);

criterion_main!(default_config);
