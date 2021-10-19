use std::future::Future;

pub trait Client {
    fn start() -> dyn Future<Output=()>;
}
