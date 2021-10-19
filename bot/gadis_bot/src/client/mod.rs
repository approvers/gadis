use std::future::Future;

pub trait Client {
    fn new() -> Self;
    fn start() -> dyn Future<Output=()>;
}
