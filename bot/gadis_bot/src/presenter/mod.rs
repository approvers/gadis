use crate::domain::DiscordUserInformation;

pub trait DiscordUserProvider {
    fn provide(discord_user: DiscordUserInformation);
}
