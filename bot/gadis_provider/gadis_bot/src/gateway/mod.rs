mod mock;

use crate::entity::User;

trait DiscordGateway {
    type E: std::error::Error;

    fn request(&self, user_id: &str) -> Result<Option<User>, Self::E>;
}
