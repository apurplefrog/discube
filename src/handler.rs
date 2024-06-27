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
        println!("New message: {}", msg.content);
        println!("\tsent by: {}", msg.author);

        if let Some(reply_message) = reply(&msg.content) {
            if let Err(why) = msg.channel_id.say(&ctx.http, reply_message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn reply(command: &str) -> Option<String> {
    Some(command.to_string())
}
