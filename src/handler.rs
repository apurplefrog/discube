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
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        println!("New message: {}", msg.content);
        println!("\tsent by: {}", msg.author);
        if msg.content == "discube scramble Mega" {
            let scramble = &Megaminx::scramble(1)[0];
            if let Err(why) = msg.channel_id.say(&ctx.http, scramble).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
