use crate::entity::User;

trait DiscordGateway {
    fn request(&self, user_id: &str) -> User;
}
