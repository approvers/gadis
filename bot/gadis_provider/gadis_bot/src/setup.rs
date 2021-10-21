pub(crate) trait Setup {
    type E;
    fn setup() -> Result<Self, Self::E>;
}

pub(crate) fn initialize() {
    todo!("Setup things");
}
