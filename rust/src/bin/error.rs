#![feature(lazy_cell)]

use std::sync::LazyLock;

static ERR: LazyLock<anyhow::Error> =
    LazyLock::new(|| anyhow::anyhow!("original err").context("ctx"));

fn main() {
    std::env::set_var("RUST_BACKTRACE", "0");
    let err = anyhow::anyhow!(&*ERR);
    // ctx
    println!("{:?}", err);
    // source: None
    println!("source: {:?}", err.source());

    // However, it seems that we have "source" here?

    // Error {
    //     context: "ctx",
    //     source: "original err",
    // }
    println!("{:#?}", err);
}
