use crate::entity::User;

pub(crate) trait UserProviderGateway {
    fn request(id: &str) -> User;
}
