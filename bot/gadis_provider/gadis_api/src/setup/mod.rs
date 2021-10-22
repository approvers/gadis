use async_trait::async_trait;

pub(crate) mod initialize;

pub(crate) trait Setup {
    type E: std::error::Error;
    fn setup(&mut self) -> Result<(), Self::E>;
}

#[async_trait]
pub(crate) trait Start {
    type E: std::error::Error;
    async fn start(self) -> Result<(), Self::E>;
}
