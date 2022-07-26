use crate::BOT_NAME;
use clap::{Args, Parser, Subcommand};

use regex::Regex;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    http::CacheHttp,
    json::json,
    model::{channel::Message, gateway::Ready},
};

//// Event Handler /////////////////////////////////////////////////////////////////////////////////
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

//// Commands //////////////////////////////////////////////////////////////////////////////////////
#[group]
#[commands(rustybot)]
pub struct General;

#[derive(Parser, Debug)]
#[clap(name = BOT_NAME, author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Echos the specified message a specified amount of time
    Echo(EchoData),

    /// Delete messages starting from the latest
    Delete { count: u32 },
}

#[derive(Args, Debug)]
pub struct EchoData {
    /// message to echo
    #[clap(short, long, value_parser)]
    pub message: Option<String>,

    /// echo amount
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub count: u32,
}

#[command]
async fn rustybot(ctx: &Context, msg: &Message) -> CommandResult {
    let regex = Regex::new(r#""[^"]+"|[^!\s]+"#).unwrap();

    let args: Vec<String> = regex
        .find_iter(&msg.content)
        .filter_map(|data| {
            let quotes = Regex::new(r#"""#).unwrap();
            Some(quotes.replace_all(data.as_str(), "").to_string())
        })
        .collect();

    let mut help_str: String = "".to_string();
    match Cli::try_parse_from(args.into_iter()) {
        Ok(data) => match &data.command {
            Commands::Echo(echo) => {
                let out = match &echo.message {
                    Some(data) => data,
                    None => {
                        help_str = std::format!("run '!{} echo -h' for help.", BOT_NAME);
                        ""
                    }
                };

                if out != "" {
                    for _ in 0..(echo.count) {
                        msg.reply(ctx, std::format!("```{}```", out)).await?;
                    }
                } else {
                    msg.reply(ctx, std::format!("```{}```", help_str)).await?;
                }
            }

            Commands::Delete { count } => {
                let http = ctx.http();
                let channel_id = msg.channel_id;
                let res = channel_id
                    .messages(http, |retriver| retriver.limit(count.clone() as u64))
                    .await;

                match res {
                    Ok(data) => match channel_id.delete_messages(http, data).await {
                        Ok(_) => (),
                        Err(e) => println!("{:#?}", e),
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                };
            }
        },
        Err(e) => {
            msg.reply(ctx, std::format!("error:```{}```", e.to_string()))
                .await?;
        }
    };

    Ok(())
}
