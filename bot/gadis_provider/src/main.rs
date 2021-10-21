extern crate gadis_api;
extern crate gadis_bot;

use gadis_api::api_entry;
use gadis_bot::bot_entry;

fn main() {
    println!("Hello, world!");
    api_entry();
    bot_entry();
}
