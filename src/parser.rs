use crate::BOT_NAME;
use clap::{Args, Parser, Subcommand};

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

    /// Delete messages starting from the latest [1 to 99]
    Clear(ClearData),

    /// Take a screenshot of a website
    View(ViewData),
}

#[derive(Args, Debug)]
pub struct EchoData {
    /// message to echo
    #[clap(short, long, value_parser)]
    pub message: String,

    /// echo amount
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub count: u32,
}

#[derive(Args, Debug)]
pub struct ClearData {
    /// echo amount
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub count: u32,

    /// use a raw loop instead of a single api call
    #[clap(long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ViewData {
    /// Url of the website
    #[clap(short, long, value_parser)]
    pub url: String,

    #[clap(short = 'x', long, value_parser, default_value_t = 1920)]
    pub width: u16,

    #[clap(short = 'y', long, value_parser, default_value_t = 1080)]
    pub height: u16,
}
