mod setup;
mod controller;
mod entity;
mod presenter;
mod usecase;

use crate::setup::Start;

pub async fn setup() {
    let handler = setup::initialize::initialize();
    handler.start().await;
}
