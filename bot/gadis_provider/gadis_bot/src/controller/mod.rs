use crate::entity::User;

pub trait DiscordUserProvideRequestHandler {
    fn request(discord_user: &str) -> User;
}
