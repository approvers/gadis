use crate::entity::User;
use crate::controller::DiscordUserProvideRequestHandler;

struct Dummy;
impl DiscordUserProvideRequestHandler for Dummy {
    fn request(&self, _id: &str) -> User { panic!("Dummy called"); }
}

pub(crate) fn initialize() -> impl DiscordUserProvideRequestHandler {
    Dummy
}
