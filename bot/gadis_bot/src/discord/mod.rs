use crate::domain::DiscordUserInformation;

pub trait DiscordGateway {
    fn request_user(discord_id: u32) -> DiscordUserInformation;
}
