pub trait DiscordUserRequestor {
    fn new() -> Self;
    fn request(discord_id: i32);
}
