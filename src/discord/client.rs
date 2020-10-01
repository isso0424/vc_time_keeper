use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::prelude::{Client, EventHandler};

struct Handler;

impl EventHandler for Handler {}

#[allow(dead_code)]
pub fn start(token: String) -> CommandResult {
    let mut client = Client::new(token, Handler)?;
    client.with_framework(StandardFramework::new().configure(|c| c.prefix("!")));

    if let Err(why) = client.start() {
        println!(
            "Error ocurred!!!\n-----------\nReason\n-----------\n{}",
            why
        );
    }

    Ok(())
}
