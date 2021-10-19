use std::future::Future;

trait Client {
    fn new() -> Self;
    fn start() -> dyn Future<Output=()>;
}
