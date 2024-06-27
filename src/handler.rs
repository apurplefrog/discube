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
        if let Some(reply_message) = reply(&msg) {
            if let Err(why) = msg.channel_id.say(&ctx.http, reply_message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn reply(msg: &Message) -> Option<String> {
    let command_words: Vec<&str> = msg.content.split_whitespace().collect();
    if command_words[0] != "discube" {
        return None;
    }

    let function = command_words[1];

    if function == "dailyscram" {
        let clock_scrambles = clock::scramble(5);
        let mega_scrambles = megaminx::scramble(5);

        let message = format!(
            "# Daily Competition #1 🔥\n**Clock Scrambles:**\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n**Megaminx Scrambles:**\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}",
            clock_scrambles[0],
            clock_scrambles[1],
            clock_scrambles[2],
            clock_scrambles[3],
            clock_scrambles[4],
            mega_scrambles[0],
            mega_scrambles[1],
            mega_scrambles[2],
            mega_scrambles[3],
            mega_scrambles[4]
        );

        return Some(message);
    }

    Some("uh".to_string())
}
