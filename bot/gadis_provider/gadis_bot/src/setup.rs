use crate::entity::User;
use crate::controller::DiscordUserProvideRequestHandler;

pub(crate) trait Setup {
    type E: std::error::Error;
    fn setup(&self) -> Result<(), Self::E>;
}

struct Dummy;
impl DiscordUserProvideRequestHandler for Dummy {
    fn request(&self, _id: &str) -> User { panic!("Dummy called"); }
}

pub(crate) fn initialize() -> impl DiscordUserProvideRequestHandler {
    Dummy
}
