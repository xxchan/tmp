use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 6666));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // Prepare some signal for when the server should start shutting down...
    let graceful = server.with_graceful_shutdown(async {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    });

    // Await the `server` receiving the signal...
    if let Err(e) = graceful.await {
        println!("server error: {}", e);
    } else {
        println!("server shutdown gracefully");
    }
}
