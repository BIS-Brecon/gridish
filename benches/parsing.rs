use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use gridish::{OSGB, OSI};
use std::str::FromStr;

const DIGITS: [&str; 6] = ["", "01", "0123", "012345", "01234567", "0123456789"];

pub fn from_string_osgb(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_string_osgb");

    for digits in DIGITS.iter() {
        group.throughput(Throughput::Elements(1));

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{} digits", digits.len())),
            digits,
            |b, &digits| {
                b.iter(|| OSGB::from_str(&format!("SO{}", digits)).unwrap());
            },
        );
    }

    group.finish();
}

pub fn from_string_osi(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_string_osi");

    for digits in DIGITS.iter() {
        group.throughput(Throughput::Elements(1));

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{} digits", digits.len())),
            digits,
            |b, &digits| {
                b.iter(|| OSI::from_str(&format!("O{}", digits)).unwrap());
            },
        );
    }

    group.finish();
}

criterion_group!(benches, from_string_osgb, from_string_osi);
criterion_main!(benches);
