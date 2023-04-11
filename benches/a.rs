use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn add(input: &str) -> isize {
    1 + input.parse::<isize>().unwrap()
}

fn bench_hello(c: &mut Criterion) {
    let input = "1";
    c.bench_function("hello", |b| {
        b.iter(|| {
            black_box(add(input));
        })
    });
}

fn bench_hello_2(c: &mut Criterion) {
    let input = "2";
    c.bench_function("hello", |b| {
        b.iter(|| {
            black_box(add(input));
        })
    });
}

criterion_group!(benches, bench_hello, bench_hello_2);
criterion_main!(benches);
