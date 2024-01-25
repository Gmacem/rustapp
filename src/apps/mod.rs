mod find { pub mod app; pub mod args; mod strategies; }
mod ls;

use crate::apps::ls::{ LsArgs, app::LsApp };
use crate::apps::find::{ args::FindAppArgs, app::FindApp };

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

    /// A subcommand for finding files in a directory
    Find(FindAppArgs)
}

pub fn run() -> Result<(), ()>{
    let cli = Cli::parse();
    match cli.command {
        Commands::Ls(args) => {
            let current_dir = current_dir().unwrap_or_default();
            let ls_app = LsApp::new(current_dir);
            match ls_app.print_files(io::stdout(), args) {
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("Failed to get a list of files: {}", err);
                    Err(())
                },
            }
        }
        Commands::Find(args) => {
            let current_dir = current_dir().unwrap_or_default();
            let mut find_app = FindApp::new(current_dir, args);
            match find_app.run() {
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("Failed to find files: {}", err);
                    Err(())
                }
            }
        }
    }
}
