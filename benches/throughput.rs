use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use collatz_weyl::collatz::{CollatzWeyl128, CollatzWeyl128_64, CollatzWeyl64};

fn bench_rng(c: &mut Criterion) {
    let mut group = c.benchmark_group("collatz_weyl");
    group.throughput(Throughput::Bytes(8));

    group.bench_function("64bit", |b| {
        let mut generator = CollatzWeyl64::new(12345678901234567891);
        b.iter(|| generator.next())
    });

    group.bench_function("128_64bit", |b| {
        let mut generator = CollatzWeyl128_64::new(12345678901234567891);
        b.iter(|| generator.next())
    });

    group.bench_function("128bit", |b| {
        let mut generator = CollatzWeyl128::new(12345678901234567891);
        b.iter(|| generator.next())
    });

    group.finish();
}

criterion_group!(benches, bench_rng);
criterion_main!(benches);