use serenity::builder::EditMember;
use serenity::client::Context;
use serenity::model::{
    guild::Guild,
    id::{ChannelId, GuildId, UserId},
};
use serenity::Error;

const DEAFEN_CHANNEL_ID: u64 = 696340084370178079;

pub fn kick(guild_id: GuildId, user_id: UserId, context: &Context) -> Result<(), Error> {
    let guild = Guild::get(context, guild_id)?;
    let member = guild.member(context, user_id)?;
    member.edit(context, |edit_member: &mut EditMember| {
        let channel_id: ChannelId = From::from(DEAFEN_CHANNEL_ID);
        edit_member.voice_channel(channel_id)
    })?;

    Ok(())
}
