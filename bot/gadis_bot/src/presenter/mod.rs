use crate::domain::DiscordUserInformation;

trait DiscordUserProvider {
    fn provide(discord_user: DiscordUserInformation);
}
