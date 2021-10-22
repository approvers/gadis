mod controller;
mod setup;
mod presenter;
mod usecase;

pub mod entity;

pub use controller::DiscordUserProvideRequestHandler;

pub fn setup() -> impl DiscordUserProvideRequestHandler {
    setup::initialize()
}
