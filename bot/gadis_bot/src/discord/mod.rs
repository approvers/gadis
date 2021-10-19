use crate::domain::DiscordUserInformation;

trait DiscordGateway {
    fn new() -> Self;
    fn request_user(discord_id: u32) -> DiscordUserInformation;
}
