use clap::Parser;
use tagger::cli::*;

fn main() -> Result<(), minus::error::MinusError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Tags(args) => tags_command(args),
    }
}
