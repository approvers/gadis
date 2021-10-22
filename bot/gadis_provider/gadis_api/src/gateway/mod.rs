use crate::entity::User;

pub(crate) trait UserProviderGateway {
    fn request(&self, id: &str) -> User;
}
