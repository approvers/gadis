use crate::domain::DiscordUserInformation;

trait DiscordGateway {
    fn request_user(discord_id: u32) -> DiscordUserInformation;
}
