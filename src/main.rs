mod discord;
mod timer;

use std::env;

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    discord::client::start(token.to_string()).unwrap();
}
