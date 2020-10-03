mod discord;
mod timer;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token");
    discord::client::start(token).unwrap();
}
