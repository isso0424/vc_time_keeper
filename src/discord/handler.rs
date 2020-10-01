use serenity::{gateway::Ready, prelude::EventHandler};

pub struct Handler;

impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("THIS BOT ON READY");
    }
}
