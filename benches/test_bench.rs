use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use testing_things::fib;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
