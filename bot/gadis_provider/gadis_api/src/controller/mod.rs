use async_trait::async_trait;

#[async_trait]
pub(crate) trait UserRequestHandler {
    async fn start();
}
