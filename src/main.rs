use std::{fmt::Debug, path};

use clap::{command, Parser, Subcommand};

mod commands;
mod config;
mod models;
// Dev mod
mod lib;

#[derive(Parser, Debug)]
#[command(name = "cli", version = "1.0", about = "Project Manager CLI")]
struct Cli {
    #[arg()]
    project_name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        path: path::PathBuf,

        #[arg(short, long)]
        name: Option<String>,
    },
    List,
    Delete {
        project_name: String,
    },
}

fn main() {
    if let Err(e) = crate::config::init_config() {
        eprintln!("Failed to initialize config: {:?}", e);
        std::process::exit(1);
    }

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { path, name }) => {
            crate::commands::add::execute(path, name);
        }
        Some(Commands::List) => {
            crate::commands::list::execute();
        }
        Some(Commands::Delete { project_name }) => {
            crate::commands::delete::execute(project_name);
        }
        None => {
            if let Some(project_name) = cli.project_name {
                crate::commands::base::execute(project_name);
            } else {
                eprintln!("No subcommand or project name provided");
                std::process::exit(1);
            }
        }
    }
}
