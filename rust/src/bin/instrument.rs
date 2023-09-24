use tracing::{info_span, Instrument};

async fn my_async_function() {
    let span = info_span!("my_async_function");
    async move {
        // This is correct! If we yield here, the span will be exited,
        // and re-entered when we resume.
        some_other_async_function().await;

        //more asynchronous code inside the span...
    }
    // instrument the async block with the span...
    .instrument(span)
    // ...and await it.
    .await
}

async fn some_other_async_function() {
    tracing::info!("hello")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().pretty().init();
    my_async_function().await;
}
