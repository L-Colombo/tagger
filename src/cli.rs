use crate::{config::Userconfig, io::*};
use clap::{Args, Parser, Subcommand};

// Commands and their arguments

#[derive(Parser)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print tags to stdout or to pager
    #[clap(visible_aliases = &["t", "tag"])]
    Tags(TagArgs),
}

#[derive(Args, Debug)]
#[command(about = "")]
pub struct TagArgs {
    /// Optional file to search instead of searching in the whole Org directory
    #[arg(long, short, value_name = "FILE")]
    pub file: Option<String>,
}

// Functions to be called (wrappers)

pub fn tags_command(args: TagArgs) -> Result<(), minus::MinusError> {
    let cfg: Userconfig = Userconfig::new();
    match args.file {
        None => match get_all_tags(&cfg) {
            Some(taglist) => print_tags_to_stdout_or_pager(taglist)?,
            None => println!("Could not find any tags in your org directory"),
        },
        Some(file_name) => match get_tags_from_file(&cfg, file_name.clone()) {
            Some(taglist) => print_tags_to_stdout_or_pager(taglist)?,
            None => println!("I have found no tags in file: {}", file_name),
        },
    }
    Ok(())
}
