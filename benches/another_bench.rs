use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use testing_things::fib;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 40", |b| b.iter(|| fib(black_box(40))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
