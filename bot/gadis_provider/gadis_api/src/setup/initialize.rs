use async_trait::async_trait;
use crate::setup::Start;

use crate::controller::warp::HttpUserRequestHandler;
use crate::controller::console::ConsoleUserRequestHandler;

pub(crate) fn initialize() -> impl Start {
    #[cfg(feature = "client-warp")]
    let client = HttpUserRequestHandler;

    #[cfg(feature= "client-console")]
    let client = ConsoleUserRequestHandler;

    client
}
