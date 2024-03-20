#![allow(unused_imports)]
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    return run().await;
}
