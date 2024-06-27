use serenity::{all::ApplicationId, model::gateway::GatewayIntents, prelude::*};
use std::fs;

use crate::handler;

pub async fn connect() {
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
    .event_handler(handler::Handler)
    .application_id(application_id)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
