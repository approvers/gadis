use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_trait::async_trait;
use warp::Filter;

use super::UserRequestHandler;
use crate::setup::{Setup, Start};

pub(crate) struct HttpUserRequestHandler;

impl UserRequestHandler for HttpUserRequestHandler {}

#[async_trait]
impl Start for HttpUserRequestHandler {
    type E = warp::Error;

    async fn start(self) -> Result<(), Self::E> {
        let route = warp::path!("user" / u32)
            .map(|id| format!("Requested: {}", id));

        warp::serve(route)
            .bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080))
            .await;

        Ok(())
    }
}
