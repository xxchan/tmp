#![feature(type_alias_impl_trait)]

use std::time::Duration;

use aws_sdk_s3::{
    config::{Credentials, Region},
    Client,
};
use futures::Future;
use hyper::client::HttpConnector;
use tokio::runtime::Builder;
use tracing_subscriber::fmt::format::{FmtSpan, Full};

fn main() {
    let format = tracing_subscriber::fmt::format().pretty();

    tracing_subscriber::fmt()
        .event_format(format)
        .with_span_events(FmtSpan::FULL)
        .init();

    run();
}

#[tracing::instrument]
fn run() {
    tracing::info!("run!");
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("frontend-compute-threads")
        .enable_all()
        .build()
        .unwrap();

    runtime.spawn(async {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            tracing::info!("loop");
        }
    });

    // spawn_blocking won't stuck!!!
    // runtime.spawn_blocking(|| {
    // spawn will stuck at 128
    runtime.spawn(async {
        tracing::info!("spawned task");

        ok_1(my_task(130));
        ok_2(my_task(130));
        ok_2_2(my_task(130));
        ok_3(my_task(130));

        block_0(my_task(130));
        panic_1(my_task(130));

        tracing::info!("task DONE!!!!!");
    });
    std::thread::sleep(Duration::from_secs(1000));
}

fn block_0(fut: impl Future<Output = ()>) {
    futures::executor::block_on(async {
        fut.await;
    });
}

fn panic_1(fut: impl Future<Output = ()>) {
    // panic: Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.

    tokio::runtime::Handle::current().block_on(async {
        fut.await;
    });
}

fn ok_1(fut: impl Future<Output = ()>) {
    futures::executor::block_on(
        // add unconstrained can also solve the problem...
        tokio::task::unconstrained(async {
            fut.await;
        }),
    );
}

fn ok_2(fut: impl Future<Output = ()>) {
    tokio::task::block_in_place(|| {
        // If we have block_in_place, either block_on is ok
        tokio::runtime::Handle::current().block_on(async {
            fut.await;
        })
    });
}

fn ok_2_2(fut: impl Future<Output = ()>) {
    tokio::task::block_in_place(|| {
        // If we have block_in_place, either block_on is ok
        futures::executor::block_on(async {
            fut.await;
        })
    });
}

fn ok_3(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::runtime::Handle::current().spawn_blocking(|| {
        // If we have block_in_place, either block_on is ok
        futures::executor::block_on(async {
            fut.await;
        })
    });
}

async fn my_task(n: usize) {
    for i in 0..n {
        pause_and_return(i).await;
    }
}

async fn pause_and_return(n: usize) -> usize {
    println!("Starting {}", n);
    // This immediately makes it dead
    // for spawn_blocking, it won't dead! ok_3 is also ok
    // tokio::task::yield_now().await;
    tokio::time::sleep(Duration::from_millis(1)).await;
    println!("Finished {}", n);
    n
}
