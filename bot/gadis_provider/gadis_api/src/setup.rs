use async_trait::async_trait;
use crate::controller::UserRequestHandler;

struct Dummy;

#[async_trait]
impl UserRequestHandler for Dummy {
    async fn start(self) { unimplemented!(); }
}

pub(crate) fn initialize() -> impl UserRequestHandler {
    Dummy
}
