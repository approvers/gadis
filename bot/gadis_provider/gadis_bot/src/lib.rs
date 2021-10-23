mod controller;
mod setup;
mod presenter;
mod usecase;
mod gateway;

pub mod entity;

pub use controller::DiscordUserProvideRequestHandler;

pub fn setup() -> impl DiscordUserProvideRequestHandler {
    setup::initialize::initialize()
}
