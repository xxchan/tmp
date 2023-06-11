#[tokio::main]
async fn main() {
    println!("Hello async world!");
    // panic!("Oh no!")
    let x = r[1];
    // trigger a SIGSEGV
    unsafe {
        println!("{}", x.get_unchecked(10000000));
    }
}

