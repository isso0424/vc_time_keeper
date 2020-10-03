use serenity::builder::EditMember;
use serenity::client::Context;
use serenity::model::{
    guild::Guild,
    id::{ChannelId, GuildId, UserId},
};
use serenity::Error;
use std::env;

pub fn kick(guild_id: GuildId, user_id: UserId, context: &Context) -> Result<(), Error> {
    let channel_id: u64 = env::var("CHANNEL_ID").expect("0").parse().expect("0");
    let guild = Guild::get(context, guild_id)?;
    let member = guild.member(context, user_id)?;
    member.edit(context, |edit_member: &mut EditMember| {
        let channel_id: ChannelId = From::from(channel_id);
        edit_member.voice_channel(channel_id)
    })?;

    Ok(())
}
