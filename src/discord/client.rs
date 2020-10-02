use crate::discord::action::kick;
use crate::timer::event_loop::lazy_event;
use chrono::offset::Local;
use chrono::{Datelike, Duration, NaiveTime, TimeZone};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::prelude::{Client, EventHandler};
use std::convert::TryFrom;

#[group]
#[commands(set)]
struct General;

struct Handler;

impl EventHandler for Handler {}

pub fn start(token: String) -> CommandResult {
    let mut client = Client::new(&token, Handler).expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!vcTimer "))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!(
            "Error ocurred!!!\n-----------\nReason\n-----------\n{}",
            why
        );
    }

    Ok(())
}

#[command]
pub fn set(context: &mut Context, message: &Message) -> CommandResult {
    let content: Vec<&str> = message.content.split_whitespace().collect();
    if content.len() < 3 {
        return Ok(());
    }

    let raw_date = match content.iter().nth(2) {
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

    println!("{}", duration);

    if let Some(guild_id) = message.guild_id {
        let user_id = message.author.id;
        lazy_event(duration, guild_id, user_id, context, kick);
    }
    Ok(())
}

fn get_duration(raw_date: &str) -> Option<Duration> {
    let now = Local::now();
    let time = match Local.datetime_from_str(
        format!("{}-{}-{} {}", now.year(), now.month(), now.day(), raw_date).as_str(),
        "%F %R",
    ) {
        Ok(date_time) => date_time.time(),
        Err(_) => return None,
    };
    let mut duration = NaiveTime::signed_duration_since(time, now.time());
    if duration.num_seconds() < 0 {
        duration = duration + Duration::hours(24);
    }

    Some(duration)
}
