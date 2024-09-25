use std::{fmt::Debug, path};

use clap::{command, Parser, Subcommand};

mod commands;
mod config;
mod models;

#[derive(Parser, Debug)]
#[command(name = "cli", version = "1.0", about = "Project Manager CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        path: path::PathBuf,

        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() {
    if let Err(e) = crate::config::init_config() {
        eprintln!("Failed to initialize config: {:?}", e);
        std::process::exit(1);
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path, name } => {
            crate::commands::add::execute(path, name);
        }
    }
}
