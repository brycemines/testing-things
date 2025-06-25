use kdam::tqdm;

fn thing(chars: Vec<&str>) {
    let mut charset = String::new();

    for i in tqdm!(chars.iter()) {
        charset += i;
    }

    eprintln!();
}

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("kdam progress bar", |b| b.iter(|| black_box(thing(vec!["a", "b", "c", "d"]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
