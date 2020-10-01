use crate::discord::handler;
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
use serenity::prelude::Client;

pub fn start(token: String) -> CommandResult {
    let mut client = Client::new(token, handler::Handler)?;
    client.with_framework(StandardFramework::new().configure(|c| c.prefix("!")));

    if let Err(why) = client.start() {
        println!(
            "Error ocurred!!!\n-----------\nReason\n-----------\n{}",
            why
        );
    }

    Ok(())
}
