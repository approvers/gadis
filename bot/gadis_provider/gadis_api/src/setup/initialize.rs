use async_trait::async_trait;
use crate::controller::UserRequestHandler;

struct Dummy;
impl UserRequestHandler for Dummy {}

pub(crate) fn initialize() -> impl UserRequestHandler {
    Dummy
}
