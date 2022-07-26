mod event_handler;
mod helpers;
mod parser;

use dotenv::dotenv;
use std::env;

use event_handler::{Handler, GENERAL_GROUP};
use serenity::{framework::standard::StandardFramework, prelude::*};

static BOT_NAME: &'static str = "xyz";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(
        &env::var("DISCORD_TOKEN").expect("Discord token env variable not defined!"),
        intents,
    )
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
