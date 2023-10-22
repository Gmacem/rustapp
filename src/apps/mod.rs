mod ls;

use crate::apps::ls::app::LsApp;
use crate::apps::ls::LsArgs;
use clap::{Parser, Subcommand};
use log::error;
use std::env::current_dir;
use std::io;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// A subcommand for showing a list of files by path
    Ls(LsArgs),
}

pub fn run() -> Result<(), ()>{
    let cli = Cli::parse();
    match &cli.command {
        Commands::Ls(args) => {
            let current_dir = current_dir().unwrap_or_default();
            let ls_app = LsApp::new(current_dir);
            match ls_app.print_files(io::stdout(), args.path.clone()) {
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("Failed to get a list of files: {}", err);
                    return Err(());
                },
            }
        }
    }
}