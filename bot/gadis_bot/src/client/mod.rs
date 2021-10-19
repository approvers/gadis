use std::future::Future;
use async_trait::async_trait;

pub mod console;

#[async_trait]
pub trait Client {
    fn new() -> Self;
    async fn start(&self);
}
