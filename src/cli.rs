use crate::{config::Userconfig, io::*, locate::locate, refile, search::search_tags};
use bat::PrettyPrinter;
use clap::{Args, Parser, Subcommand, ValueHint, arg, builder::styling};
use minus::{MinusError, Pager, page_all};
use std::{io::Write, process::exit};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

// Commands and their arguments
// Commands

#[derive(Parser)]
#[command(name = "tgr")]
#[command(
    version,
    about = "Manage `.org` files' tags from the CLI",
    long_about = "Search and visualize tags in your `.org` files"
)]
#[command(styles = STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Locate the files that contain a tag matching <PATTERN>
    #[clap(aliases = &["l", "loc"])]
    Locate(LocateArgs),
    /// Refile org trees that have tags that match a pattern
    #[clap(aliases = &["r", "ref"])]
    Refile(RefileArgs),
    /// Search tags in Org directory or file
    #[clap(alias = "s")]
    Search(SearchArgs),
    /// Print tags to stdout or to pager
    #[clap(aliases = &["t", "tag"])]
    Tags(TagArgs),
}

// Args
#[derive(Args, Debug)]
pub struct LocateArgs {
    /// Pattern to search for tags
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// Match the pattern strictly or loosely
    #[arg(short, long, value_name = "STRICT")]
    pub strict: bool,
}

#[derive(Args, Debug)]
pub struct RefileArgs {
    /// Pattern to find Org trees to refile
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// Print the contents to stdout instead of pager
    #[arg(short, long, value_name = "NO PAGER")]
    pub no_pager: bool,
    /// Name of the output file. If not given, ouptut is paged to the console
    #[arg(value_name = "OUTPUT FILE")]
    pub output_file: Option<String>,
    /// Match the pattern strictly or loosely
    #[arg(short, long, value_name = "STRICT")]
    pub strict: bool,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Pattern used to search for tags
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// File where to search for tags
    #[arg(long, short, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<String>,
    /// Force the output to a pager
    #[arg(long, short, value_name = "PAGER")]
    pub pager: bool,
}

#[derive(Args, Debug)]
pub struct TagArgs {
    /// Optional file to search instead of searching in the whole Org directory
    #[arg(long, short, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<String>,
    /// Force the output to a pager
    #[arg(long, short, value_name = "PAGER")]
    pub pager: bool,
}

// Wrappers and helpers
pub fn locate_command(args: LocateArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    let files: Vec<String> = locate(args.pattern, cfg, args.strict);

    for file in files {
        println!("{file}")
    }

    Ok(())
}

pub fn refile_command(args: RefileArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    let file_contents: String = refile::refile(args.pattern, cfg, args.strict);

    match args.output_file {
        None => {
            if args.no_pager {
                println!("{file_contents}");
                Ok(())
            } else {
                let mut pager = Pager::new();

                let _ = PrettyPrinter::new()
                    .input_from_bytes(file_contents.as_bytes())
                    .language("org")
                    .print_with_writer(Some(&mut pager));

                let _ = Pager::set_run_no_overflow(&pager, true);

                page_all(pager)
            }
        }
        Some(output_file) => {
            let fname = if output_file.ends_with(".org") {
                output_file
            } else {
                format!("{output_file}.org")
            };

            let mut output_file = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(fname)
                .expect("Something went wrong creating the refiled file");

            match output_file.write_all(file_contents.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => {
                    eprintln!("Could not write the refiled file");
                    exit(1)
                }
            }
        }
    }
}

pub fn search_command(args: SearchArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    match search_tags(args.pattern, &cfg, args.file) {
        None => println!("No tags matching the provided pattern were found!"),
        Some(taglist) => print_tags_to_stdout_or_pager(taglist, args.pager)?,
    }
    Ok(())
}

pub fn tags_command(args: TagArgs) -> Result<(), MinusError> {
    let cfg: Userconfig = Userconfig::new();
    match args.file {
        None => match get_all_tags(&cfg) {
            Some(taglist) => print_tags_to_stdout_or_pager(taglist, args.pager)?,
            None => println!("Could not find any tags in your org directory"),
        },
        Some(file_name) => match get_tags_from_file(&cfg, file_name.clone()) {
            Some(taglist) => print_tags_to_stdout_or_pager(taglist, args.pager)?,
            None => println!("I have found no tags in file: {file_name}"),
        },
    }
    Ok(())
}
