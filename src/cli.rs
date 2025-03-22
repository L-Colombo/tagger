use std::io::Write;

use crate::{config::Userconfig, io::*, refile, search::search_tags};
use clap::{Args, Parser, Subcommand};
use minus::MinusError;

// Commands and their arguments
// Commands

#[derive(Parser)]
#[command(name = "tagger")]
#[command(
    version,
    about = "Manage `.org` files' tags from the CLI",
    long_about = "Search and visualize tags in your `.org` files"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Refile org trees that have tags that match a pattern
    #[clap(visible_alias = "r")]
    Refile(RefileArgs),
    /// Search tags in Org directory or file
    #[clap(visible_alias = "s")]
    Search(SearchArgs),
    /// Print tags to stdout or to pager
    #[clap(visible_aliases = &["t", "tag"])]
    Tags(TagArgs),
}

// Args
#[derive(Args, Debug)]
pub struct RefileArgs {
    /// Pattern to find Org trees to refile
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// Output file
    #[arg(value_name = "OUTPUT FILE")]
    pub output_file: String,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Pattern used to search for tags
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// File where to search for tags
    #[arg(long, short, value_name = "FILE")]
    pub file: Option<String>,
}

#[derive(Args, Debug)]
#[command(about = "")]
pub struct TagArgs {
    /// Optional file to search instead of searching in the whole Org directory
    #[arg(long, short, value_name = "FILE")]
    pub file: Option<String>,
}

// Wrappers and helpers
pub fn refile_command(args: RefileArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    let file_contents: String = refile::refile(args.pattern, cfg);
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(args.output_file)
        .unwrap();

    output_file.write_all(file_contents.as_bytes()).unwrap();

    Ok(())
}

pub fn search_command(args: SearchArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    match search_tags(args.pattern, &cfg, args.file) {
        None => println!("No tags matching the provided pattern were found!"),
        Some(taglist) => print_tags_to_stdout_or_pager(taglist)?,
    }

    Ok(())
}

pub fn tags_command(args: TagArgs) -> Result<(), MinusError> {
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
