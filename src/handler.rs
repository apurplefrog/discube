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
        if let Some(reply_messages) = reply(&msg) {
            for reply_message in reply_messages {
                if let Err(why) = msg.channel_id.say(&ctx.http, reply_message).await {
                    println!("Error sending message: {:?}", why);
                }
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
    let command_words = &msg.content.split_whitespace().collect::<Vec<&str>>()[1..];
    let function = command_words[0];

    match function {
        "dailyscram" => {
            if let Ok(number) = command_words[1].parse::<u32>() {
                generate_daily_scrambles(number)
            } else {
                vec!["must be a valid number lmao".to_string()]
            }
        }
        _ => vec![format!("Unknown function: {}", function)],
    }
}

fn format_scramble_vector(scramble_vector: Vec<String>) -> String {
    scramble_vector
        .iter()
        .enumerate()
        .map(|(i, scramble)| format!("{}. {}", i, scramble))
        .collect::<String>()
}

fn generate_daily_scrambles(number: u32) -> Vec<String> {
    //let cube1 = Cube::random_cube();
    let cube1 = Cube::Megaminx;
    //let cube2 = Cube::random_cube();
    let cube2 = Cube::Clock;

    let scrambles1 = cube1.scramble(cube1.average_scramble_count());
    let scrambles2 = cube2.scramble(cube2.average_scramble_count());

    let intro = format!(
        "# Daily Comp #{} 🔥\nToday's events are **{}** and **{}**",
        number,
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
