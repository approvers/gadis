mod initialize;

pub(crate) trait Setup {
    type E: std::error::Error;
    fn setup(&mut self) -> Result<(), Self::E>;
}

pub(crate) trait Start {
    type E: std::error::Error;
    fn start(self) -> Result<(), Self::E>;
}
