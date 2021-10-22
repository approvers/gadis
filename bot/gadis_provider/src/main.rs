#[tokio::main]
async fn main() {
    println!("Hello, world!");

    gadis_api::setup().await;
    gadis_bot::setup();
}
