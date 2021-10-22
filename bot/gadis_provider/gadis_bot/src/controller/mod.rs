use crate::entity::User;

pub trait DiscordUserProvideRequestHandler {
    fn request(&self, discord_user: &str) -> User;
}
