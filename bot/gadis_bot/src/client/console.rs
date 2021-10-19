use super::Client;
use async_trait::async_trait;

pub struct ConsoleClient;

#[async_trait]
impl Client for ConsoleClient {

    fn new() -> Self { Self }

    async fn start(&self) {
        println!("This is the console client for testing upper layer.");
        println!("Currently void is implemented, so I cannot do anything now.");
        println!("Pull me later, please...");
    }
}
