use crate::entity::User;

pub(crate) trait ProvideUserRequest {
    fn request(user_id: &str) -> User;
}
