use regex::Regex;

use super::commands::parse_command;
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
        println!("{} is connected!", ready.user.name);
    }
}

//// Commands //////////////////////////////////////////////////////////////////////////////////////
#[group]
#[commands(xyz)]
pub struct General;

pub struct DiscordHandle<'a> {
    pub ctx: &'a Context,
    pub msg: &'a Message,
}

#[command]
async fn xyz(ctx: &Context, msg: &Message) -> CommandResult {
    let handle = DiscordHandle {
        ctx: &ctx,
        msg: &msg,
    };

    let args = shape_commands(&handle.msg.content.as_str());
    parse_command(&handle, &args).await;

    Ok(())
}

fn shape_commands<'a>(msg: &'a str) -> Vec<String> {
    Regex::new(r#""[^"]+"|'[^']+'|[^!\s]+"#)
        .expect("Invalid regex pattern!")
        .find_iter(&msg)
        .filter_map(|data| {
            let quotes = Regex::new(r#"""#).expect("Invalid regex pattern!");
            Some(quotes.replace_all(data.as_str(), "").to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn shape_commands() {
        let messages = super::shape_commands(r#"!start "double quotes's" mid end"#);
        assert_eq!(messages, vec!["start", "double quotes's", "mid", "end"]);
    }
}
