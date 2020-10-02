use serenity::client::Context;
use serenity::model::id::{GuildId, UserId};
use serenity::Error;
use std::thread::sleep;
use std::time::Duration;

pub fn lazy_event(
    sleep_sec: u64,
    guild_id: GuildId,
    member_id: UserId,
    context: &Context,
    event: fn(GuildId, UserId, &Context) -> Result<(), Error>,
) {
    let duration = Duration::from_secs(sleep_sec);
    sleep(duration);
    let _ = event(guild_id, member_id, context);
}
