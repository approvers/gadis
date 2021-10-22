use crate::entity::User;

pub(crate) trait UserRequestOutputPort {
    fn set_value(&mut self, user: User);
    fn get_value(self) -> User;
}
