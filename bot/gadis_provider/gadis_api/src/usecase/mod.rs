use crate::presenter::UserRequestOutputPort;

pub(crate) struct RequestUsecase;

impl RequestUsecase {
    pub fn request(&self, user_id: &str, output_port: &mut impl UserRequestOutputPort) {
        todo!("Implement here");
    }
}
