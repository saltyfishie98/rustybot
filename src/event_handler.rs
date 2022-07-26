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
    model::{channel::Message, gateway::Ready},
};

fn cli_message(msg: &String) -> String {
    std::format!("```{}```", msg)
}

fn cli_error(msg: &String) -> String {
    std::format!("```ERROR: {}```", msg)
}

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
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Echos the specified message a specified amount of time
    Echo(EchoData),

    /// Delete messages starting from the latest [1 to 99]
    Clear(ClearData),
}

#[derive(Args, Debug)]
struct EchoData {
    /// message to echo
    #[clap(short, long, value_parser)]
    message: String,

    /// echo amount
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u32,
}

#[derive(Args, Debug)]
struct ClearData {
    /// echo amount
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u32,

    /// use a raw loop instead of a single api call
    #[clap(long)]
    force: bool,
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

    match Cli::try_parse_from(args.into_iter()) {
        Ok(data) => match &data.command {
            Commands::Echo(echo) => {
                for _ in 0..(echo.count) {
                    let res = msg.reply(ctx, cli_message(&echo.message)).await;

                    match res {
                        Ok(_) => (),
                        Err(e) => println!("{}", e),
                    }
                }
            }

            Commands::Clear(data) => {
                let http = ctx.http();
                let channel_id = msg.channel_id;

                let res = channel_id
                    .messages(http, |retriver| {
                        retriver.limit((data.count.clone() as u64) + 1)
                    })
                    .await;

                if data.force {
                    if let Ok(ids) = res {
                        for id in ids {
                            if let Err(e) = channel_id.delete_message(http, id).await {
                                let res = msg.reply(http, cli_error(&e.to_string())).await;
                                match res {
                                    Ok(_) => (),
                                    Err(e) => println!("{}", e),
                                }
                            };
                        }
                    }
                } else {
                    match res {
                        Ok(ids) => {
                            if let Err(e) = channel_id.delete_messages(http, ids).await {
                                let res = msg.reply(http, cli_error(&e.to_string())).await;
                                match res {
                                    Ok(_) => (),
                                    Err(e) => println!("{}", e),
                                }
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    };
                }
            }
        },
        Err(e) => {
            let res = msg.reply(ctx, cli_message(&e.to_string())).await;

            match res {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
    };

    Ok(())
}
