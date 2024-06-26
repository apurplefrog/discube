mod scramblers;

use crate::scramblers::Megaminx;

use serenity::{
    all::ApplicationId,
    async_trait,
    model::{
        channel::Message,
        gateway::{GatewayIntents, Ready},
    },
    prelude::*,
};
use std::fs;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        println!("New message: {}", msg.content);
        println!("\tsend by: {}", msg.author);
        if msg.content == "discube scramble Mega" {
            let scramble = &Megaminx::scramble(1)[0];
            if let Err(why) = msg.channel_id.say(&ctx.http, scramble).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let tokensecret_file = fs::read("./.tokensecret").expect("Cannot read from file");
    let token = std::str::from_utf8(&tokensecret_file)
        .expect("Must be valid utf-8")
        .trim();
    let idsecret_file = fs::read("./.idsecret").expect("Cannot read from file");
    let id: u64 = std::str::from_utf8(&idsecret_file)
        .expect("Must be valid utf-8")
        .trim()
        .parse()
        .expect("Must be valid u64");

    let application_id = ApplicationId::new(id);

    let mut client = Client::builder(
        &token,
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .application_id(application_id)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
