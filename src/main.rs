use std::path;

use clap::{command, Parser, Subcommand};

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
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path, name } => {
            println!("Adding project with path: {path:?}");

            if let Some(project_name) = name {
                println!("Project name: {project_name}")
            } else {
                println!("No project name was given")
            }
        }
    }
}
