use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use gridish::Precision;
use gridish::{OSGB, OSI};

const PRECISIONS: [Precision; 6] = [
    Precision::_100Km,
    Precision::_10Km,
    Precision::_1Km,
    Precision::_100M,
    Precision::_10M,
    Precision::_1M,
];

const EASTINGS: u32 = 123_456;
const NORTHINGS: u32 = 234_567;

pub fn to_string_osgb(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_string_osgb");

    for precision in PRECISIONS.iter() {
        group.throughput(Throughput::Elements(1));

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{} digits", precision.digits())),
            precision,
            |b, &precision| {
                b.iter(|| {
                    OSGB::new(EASTINGS, NORTHINGS, precision)
                        .unwrap()
                        .to_string()
                });
            },
        );
    }

    group.finish();
}

pub fn to_string_osi(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_string_osi");

    for precision in PRECISIONS.iter() {
        group.throughput(Throughput::Elements(1));

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{} digits", precision.digits())),
            precision,
            |b, &precision| {
                b.iter(|| {
                    OSI::new(EASTINGS, NORTHINGS, precision)
                        .unwrap()
                        .to_string()
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, to_string_osgb, to_string_osi);
criterion_main!(benches);
