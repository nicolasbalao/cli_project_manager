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
    config::setup_environment().expect("Failed to setup the environment");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path, name } => {
            crate::commands::add::execute(path, name);
        }
    }
}
