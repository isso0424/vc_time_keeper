use serenity::builder::EditMember;
use serenity::client::Context;
use serenity::model::{
    guild::Guild,
    id::{ChannelId, GuildId, UserId},
};
use serenity::Error;

pub fn kick(guild_id: GuildId, user_id: UserId, context: &Context) -> Result<(), Error> {
    let guild = Guild::get(context, guild_id)?;
    let member = guild.member(context, user_id)?;
    member.edit(context, |edit_member: &mut EditMember| {
        edit_member.voice_channel(ChannelId::default())
    })?;

    Ok(())
}
