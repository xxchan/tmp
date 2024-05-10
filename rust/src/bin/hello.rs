use std::collections::HashMap;

use educe::Educe;
use futures::{
    stream::{FuturesOrdered, FuturesUnordered},
    Future, StreamExt,
};

async fn foo() {
    // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    // tokio::time::sleep(std::time::Duration::from_micros(1)).await;
}

fn wrap_fut(fut: impl Future<Output = ()>) -> impl Future<Output = ()> {
    fut
}

async fn join_parallel<T: Send + 'static>(
    futs: impl IntoIterator<Item = impl Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let tasks: Vec<_> = futs.into_iter().map(tokio::spawn).collect();
    // unwrap the Result because it is introduced by tokio::spawn()
    // and isn't something our caller can handle
    futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

#[derive(Educe, Debug, Eq)]
#[educe(Default)]
pub struct S {
    #[educe(Default(expression = u32::MAX - 1))]
    a: u32,
}

struct A;

struct B {
    a: A,
}

trait D {}

fn assert_impl<T: D>() {}

#[tokio::main]
async fn main() {
    assert_impl::<A>();

    println!(
        "S::default() : {:?}, u32::MAX - 1: {}",
        S::default(),
        u32::MAX - 1
    );

    let t = std::time::Instant::now();
    async {
        foo().await;
    }
    .await;
    // async { async { foo().await } }.await;
    println!("1: {:?}", t.elapsed());

    let t = std::time::Instant::now();
    async {
        foo().await;
    }
    .await;
    async {
        foo().await;
    }
    .await;
    println!("2: {:?}", t.elapsed());

    let t = std::time::Instant::now();
    for i in 0..10 {
        foo().await;
    }
    println!("sequential: {:?}", t.elapsed());

    let t = std::time::Instant::now();
    futures::future::join_all((0..10).map(|_| foo())).await;
    println!("join_all: {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let mut stream = (0..10).map(|_| foo()).collect::<FuturesOrdered<_>>();
    while let Some(_) = stream.next().await {}
    println!("ordered stream: {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let mut stream = (0..10).map(|_| foo()).collect::<FuturesUnordered<_>>();
    while let Some(_) = stream.next().await {}
    println!("unordered stream: {:?}", t.elapsed());

    let t = std::time::Instant::now();
    join_parallel((0..10).map(|_| foo())).await;
    println!("join_parallel: {:?}", t.elapsed());
}
