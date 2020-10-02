use crate::discord::action::kick;
use crate::timer::event_loop::lazy_event;
use chrono::offset::Local;
use chrono::Duration;
use chrono::{NaiveTime, TimeZone};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use std::convert::TryFrom;

#[command]
pub fn set(context: &mut Context, message: &Message) -> CommandResult {
    let content: Vec<&str> = message.content.split_whitespace().collect();
    if content.len() < 2 {
        return Ok(());
    }
    let raw_date = match content.iter().nth(1) {
        Some(date) => date,
        None => return Ok(()),
    };

    let duration: u64 = match get_duration(raw_date) {
        Some(duration) => match TryFrom::try_from(duration.num_seconds()) {
            Ok(duration) => duration,
            Err(_) => return Ok(()),
        },
        None => return Ok(()),
    };

    if let Some(guild_id) = message.guild_id {
        let user_id = message.author.id;
        let _ = lazy_event(duration, guild_id, user_id, context, kick);
    }
    Ok(())
}

fn get_duration(raw_date: &str) -> Option<Duration> {
    let time = match Local.datetime_from_str(raw_date, "%H:%M") {
        Ok(date_time) => date_time.time(),
        Err(_) => return None,
    };
    let now = Local::now().time();
    let mut duration = NaiveTime::signed_duration_since(time, now);
    if duration.num_seconds() < 0 {
        duration = duration + Duration::hours(24);
    }

    Some(duration)
}
