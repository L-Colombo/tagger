use clap::Parser;
use clap_allgen::*;
use std::process::exit;
use tagger::cli::*;

fn main() -> Result<(), minus::error::MinusError> {
    // Uncomment and run once to update the man pages and shell completions scripts
    // _generate_manpages_and_completions();

    let cli = Cli::parse();

    match cli.command {
        Commands::Locate(args) => locate_command(args),
        Commands::Refile(args) => refile_command(args),
        Commands::Search(args) => search_command(args),
        Commands::Tags(args) => tags_command(args),
    }
}

fn _generate_manpages_and_completions() {
    let current_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            println!("Could not get current working directory");
            exit(1)
        }
    };

    render_manpages::<Cli>(current_dir.join("etc/man/")).unwrap();
    render_shell_completions::<Cli>(current_dir.join("etc/shell_comp")).unwrap();
}
