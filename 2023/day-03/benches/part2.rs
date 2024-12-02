use criterion::{criterion_group, criterion_main, Criterion};
use day_03::process;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_day_03_part2", |b| b.iter(|| process()));
}

criterion_group!(benches, bench);
criterion_main!(benches);
