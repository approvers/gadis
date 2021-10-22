use crate::presenter::UserProvideOutputPort;

pub(crate) struct ProvideUserUsecase;

impl ProvideUserUsecase {
    fn request(&self, user_id: &str, output_port: &mut impl UserProvideOutputPort) {
        todo!("Implement here");
    }
}
