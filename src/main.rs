use std::{
    fmt::Debug,
    fs::{self, File},
    io::Write,
    path::{self, Path, PathBuf},
};

use chrono::{DateTime, Utc};
use clap::{command, Parser, Subcommand};
use serde::Serialize;

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

            add_command(path, name);
        }
    }
}

#[derive(Serialize, Debug)]
struct ProjectConfig {
    meta_data: MetaData,
}

impl ProjectConfig {
    fn new(meta_data: MetaData) -> Self {
        ProjectConfig { meta_data }
    }
}

#[derive(Serialize, Debug)]
struct MetaData {
    project_name: String,
    creation_date_utc: String,
    path: String,
}

impl MetaData {
    fn new(path: String, project_name: String) -> MetaData {
        MetaData {
            path,
            project_name,
            creation_date_utc: Utc::now().to_string(),
        }
    }
}

fn add_command(path: &path::PathBuf, project_name: &Option<String>) {
    let path = path.canonicalize().unwrap();
    println!("Path canonicalized: {path:?}");

    let project_name = match project_name {
        Some(name) => name.clone(),
        None => String::from(path.file_name().unwrap().to_str().unwrap()),
    };

    let project_meta_data = MetaData::new(String::from(path.to_str().unwrap()), project_name);

    let project_config = ProjectConfig::new(project_meta_data);

    let project_config_path = dirs::home_dir().unwrap().join(format!(
        ".project_manager_cli/projects/{}.toml",
        project_config.meta_data.project_name
    ));
    write_project_config(&project_config, &project_config_path);



}

fn write_project_config(project_config: &ProjectConfig, config_path: &PathBuf) {
    let toml_str = toml::to_string(&project_config).unwrap();
    write_file(toml_str.as_bytes(), config_path);
}

fn write_file(content: &[u8], path: &PathBuf) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let mut file = fs::File::create(path).unwrap();

    file.write_all(content).unwrap();
    println!("Config file created at {:?}", path);
}
