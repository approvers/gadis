#[tokio::main]
async fn main() {
    println!("Hello, world!");

    gadis_api::setup()
        .await
        .unwrap();
    gadis_bot::setup();
}
