use serenity::client::Context;
use serenity::model::id::{ChannelId, GuildId};
use std::time::Duration;
use tokio::time;

#[allow(dead_code)]
pub async fn lazy_event(
    sleep_sec: u64,
    guild_id: GuildId,
    member_id: ChannelId,
    context: Context,
    event: fn(GuildId, ChannelId, Context),
) {
    let duration = Duration::from_secs(sleep_sec);
    let mut interval = time::interval(duration);
    interval.tick().await;
    event(guild_id, member_id, context);
}
