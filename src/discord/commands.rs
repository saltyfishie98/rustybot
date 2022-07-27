use super::{
    event_handler::DiscordHandle,
    helpers::{cli_error, cli_message},
};
use crate::parser::{ClearData, Cli, Commands, EchoData, ViewData};
use serenity::http::CacheHttp;
use webscreenshotlib::{screenshot_tab, write_screenshot, OutputFormat};

pub async fn parse_command<'a>(handle: &'a DiscordHandle<'a>, args: &Vec<String>) {
    use clap::Parser;
    match Cli::try_parse_from(args.into_iter()) {
        Ok(data) => match &data.command {
            Commands::Echo(data) => echo_cb(&handle, &data).await,
            Commands::Clear(data) => clear_cb(&handle, &data).await,
            Commands::View(data) => view_cb(&handle, &data).await,
        },
        Err(e) => {
            let &DiscordHandle { ctx, msg } = handle;
            match msg.reply(ctx, cli_message(&e.to_string())).await {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
    };
}

async fn echo_cb<'a>(handle: &'a DiscordHandle<'a>, data: &EchoData) {
    let &DiscordHandle { ctx, msg } = handle;
    for _ in 0..(data.count) {
        match msg.reply(ctx, cli_message(&data.message)).await {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}

async fn clear_cb<'a>(handle: &'a DiscordHandle<'a>, data: &ClearData) {
    let &DiscordHandle { ctx, msg } = handle;
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
                    match msg.reply(http, cli_error(&e.to_string())).await {
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
                    match msg.reply(http, cli_error(&e.to_string())).await {
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

async fn view_cb<'a>(handle: &'a DiscordHandle<'a>, data: &ViewData) {
    let &DiscordHandle { ctx, msg } = handle;

    let image = match screenshot_tab(
        data.url.as_str(),
        OutputFormat::PNG,
        80,
        false,
        data.width,
        data.height,
        "",
    ) {
        Ok(data) => data,
        Err(_) => vec![0],
    };

    write_screenshot("temp/images/view.png", image).expect("write error!");

    msg.channel_id
        .send_files(ctx, ["temp/images/view.png"], |m| {
            m.reference_message(msg)
                .allowed_mentions(|am| am.replied_user(true))
        })
        .await
        .expect("file upload error");
}
