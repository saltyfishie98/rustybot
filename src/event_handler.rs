use regex::Regex;

use crate::{
    helpers::{cli_error, cli_message},
    parser::{Cli, Commands},
};
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
#[commands(xyz)]
pub struct General;

#[command]
async fn xyz(ctx: &Context, msg: &Message) -> CommandResult {
    let regex = Regex::new(r#""[^"]+"|[^!\s]+"#).expect("Invalid regex pattern!");

    let args: Vec<String> = regex
        .find_iter(&msg.content)
        .filter_map(|data| {
            let quotes = Regex::new(r#"""#).expect("Invalid regex pattern!");
            Some(quotes.replace_all(data.as_str(), "").to_string())
        })
        .collect();

    use clap::Parser;
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
