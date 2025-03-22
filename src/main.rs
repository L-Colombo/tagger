use clap::Parser;
use tagger::cli::*;

fn main() -> Result<(), minus::error::MinusError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Refile(args) => refile_command(args),
        Commands::Search(args) => search_command(args),
        Commands::Tags(args) => tags_command(args),
    }
}
