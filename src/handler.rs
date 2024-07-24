use crate::scramblers::*;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected...", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let reply_messages = reply(&msg);
        if reply_messages.is_none() {
            return;
        }

        for reply_message in reply_messages.unwrap() {
            if let Err(why) = msg.channel_id.say(&ctx.http, reply_message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn reply(msg: &Message) -> Option<Vec<String>> {
    let command_words: Vec<&str> = msg.content.split_whitespace().collect();
    if command_words[0] != "discube" {
        return None;
    }

    Some(command(msg))
}

fn command(msg: &Message) -> Vec<String> {
    let command = msg.content.split_whitespace().collect::<Vec<&str>>();

    match command[1] {
        "dailyscram" => daily_scram(command),
        _ => vec![format!("Unknown command: {}", command[1])],
    }
}

fn format_scramble_vector(scramble_vector: Vec<String>) -> String {
    scramble_vector
        .iter()
        .enumerate()
        .map(|(i, scramble)| format!("{}. {}", i, scramble))
        .collect::<String>()
}

fn daily_scram(command: Vec<&str>) -> Vec<String> {
    let number = command[2].parse::<u32>();
    if number.is_err() {
        return vec!["Please input a valid positive integer!".to_string()];
    }

    //let cube1 = Cube::random_cube();
    let cube1 = Cube::Megaminx;
    //let cube2 = Cube::random_cube();
    let cube2 = Cube::Clock;

    let scrambles1 = cube1.scramble(cube1.average_scramble_count());
    let scrambles2 = cube2.scramble(cube2.average_scramble_count());

    let intro = format!(
        "# Daily Comp #{} 🔥\nToday's events are **{}** and **{}**",
        number.unwrap(),
        cube1.long_name(),
        cube2.long_name()
    );
    let formatted_scrambles1 = format!(
        "**{}** Scrambles:\n{}",
        cube1.long_name(),
        format_scramble_vector(scrambles1)
    );
    let formatted_scrambles2 = format!(
        "**{}** Scrambles:\n{}",
        cube2.long_name(),
        format_scramble_vector(scrambles2)
    );

    vec![intro, formatted_scrambles1, formatted_scrambles2]
}
