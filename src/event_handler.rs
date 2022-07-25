use crate::PREFIX;
use clap::Parser;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::{channel::Message, gateway::Ready},
};

//// Event Handler /////////////////////////////////////////////////////////////////////////////////
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

//// Commands //////////////////////////////////////////////////////////////////////////////////////
#[group]
#[commands(echo)]
pub struct General;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct EchoArgs {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    content: String,
}

#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args: Vec<&str> = msg.content.split(" ").collect();
    args.remove(0);

    match EchoArgs::try_parse_from(args.into_iter()) {
        Ok(data) => {
            msg.reply(ctx, std::format!("```{}```", data.content))
                .await?;
        }
        Err(e) => {
            let mut sentence = e.to_string();
            let mut words: Vec<&str> = sentence.split(" ").collect();
            match words.iter().position(|&word| word == "echo") {
                Some(position) => {
                    words.insert(position + 1, "[OPTIONS]");
                    words.remove(position);
                    words.insert(position, PREFIX);
                    sentence = words.join(" ");
                }
                None => {}
            };

            msg.reply(ctx, std::format!("```{}```", sentence)).await?;
        }
    };

    Ok(())
}
