mod controller;
mod setup;

pub mod entity;

pub use controller::DiscordUserProvideRequestHandler;

pub fn setup() -> impl DiscordUserProvideRequestHandler {
    setup::initialize()
}
