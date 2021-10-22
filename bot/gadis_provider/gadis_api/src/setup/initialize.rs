use async_trait::async_trait;
use crate::setup::Start;

use crate::controller::warp::HttpUserRequestHandler;

pub(crate) fn initialize() -> impl Start {
    HttpUserRequestHandler
}
