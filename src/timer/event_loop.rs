use serenity::client::Context;
use serenity::model::id::{GuildId, UserId};
use serenity::Error;
use std::time::Duration;
use tokio::time;

#[allow(dead_code)]
pub async fn lazy_event(
    sleep_sec: u64,
    guild_id: GuildId,
    member_id: UserId,
    context: &Context,
    event: fn(GuildId, UserId, &Context) -> Result<(), Error>,
) {
    let duration = Duration::from_secs(sleep_sec);
    let mut interval = time::interval(duration);
    interval.tick().await;
    let _ = event(guild_id, member_id, context);
}
