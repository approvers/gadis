use crate::domain::DiscordUserInformation;

trait DiscordUserProvider {
    fn new() -> Self;
    fn provide(discord_user: DiscordUserInformation);
}
