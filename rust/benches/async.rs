use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
}

async fn foo() {
    // sleep 1ms
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
}
async fn bar() {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
}
async fn baz() {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
}

async fn fut() {
    foo().await;
    bar().await;
    baz().await;
}
async fn fut2() {
    let f1 = async {
        foo().await;
    };
    let f2 = async {
        f1.await;
        bar().await;
    };
    let f3 = async {
        f2.await;
        baz().await;
    };

    f3.await;
}

fn bench_hello(c: &mut Criterion) {
    let runtime = runtime();
    let mut group = c.benchmark_group("My Group");

    group.bench_function("test1", |b| b.iter(|| runtime.block_on(fut())));
    group.bench_function("test2", |b| b.iter(|| runtime.block_on(fut2())));

    group.finish();
}

criterion_group!(benches, bench_hello);
criterion_main!(benches);
