use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print tags to stdout or to pager
    Tag(TagArgs),
}

#[derive(Args, Debug)]
#[command(about = "")]
pub struct TagArgs {
    #[arg(long, short, value_name = "SPECIFY FILE FROM WHICH TO GET TAGS")]
    pub file: String,
}
