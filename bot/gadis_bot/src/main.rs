mod client;
mod discord;
mod domain;
mod controller;
mod presenter;

use client::Client;
use client::console::ConsoleClient;

#[tokio::main]
async fn main() {
    let client = ConsoleClient::new();
    client.start().await;
}
