use crate::domain::DiscordUserInformation;

pub trait DiscordUserProvider {
    fn new() -> Self;
    fn provide(discord_user: DiscordUserInformation);
}
