use std::{
    fmt::Debug,
    fs::{self, File},
    io::Write,
    path::{self, PathBuf},
};

use chrono::Utc;
use clap::{command, Parser, Subcommand};
use serde::{Deserialize, Serialize};

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
    setup();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { path, name } => {
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
struct ProjectIndex {
    projects: Vec<MetaData>,
}

fn add_command(path: &path::PathBuf, project_name: &Option<String>) {
    let path = path.canonicalize().unwrap();
    let project_name = match project_name {
        Some(name) => name.clone(),
        None => String::from(path.file_name().unwrap().to_str().unwrap()),
    };

    if project_exist(&project_name, path.to_str().unwrap()) {
        println!("Project already exist");
        std::process::exit(0);
    }

    let project_meta_data = MetaData::new(String::from(path.to_str().unwrap()), project_name);

    let project_config = ProjectConfig::new(project_meta_data);

    let project_config_path = dirs::home_dir().unwrap().join(format!(
        ".project_manager_cli/projects/{}.toml",
        project_config.meta_data.project_name
    ));
    write_project_config(&project_config, &project_config_path);

    let project_index_file_path = dirs::home_dir()
        .unwrap()
        .join(".project_manager_cli/project_index.toml");

    // TODO:
    if !project_index_file_path.exists() {
        // doesn't exist
        let project_index = ProjectIndex {
            projects: vec![project_config.meta_data],
        };

        write_project_index_file(&project_index);
    } else {
        let mut project_index = get_project_index();
        project_index.projects.push(project_config.meta_data);
        write_project_index_file(&project_index);
    }
}

fn write_project_config(project_config: &ProjectConfig, config_path: &PathBuf) {
    let toml_str = toml::to_string(&project_config).unwrap();
    write_file(toml_str.as_bytes(), config_path);
    println!("Project file configuration was created: {:#?}", config_path);
}

fn get_project_index() -> ProjectIndex {
    let toml_str = fs::read_to_string(
        dirs::home_dir()
            .unwrap()
            .join(".project_manager_cli/project_index.toml"),
    )
    .expect("Failed to read the project index file");

    if toml_str.is_empty() {
        return ProjectIndex { projects: vec![] };
    };

    let project_index: ProjectIndex = toml::from_str(&toml_str).expect("Failed to from_str");

    project_index
}

fn write_project_index_file(project_index: &ProjectIndex) {
    let toml_str = toml::to_string(&project_index).unwrap();
    let project_index_file_path = dirs::home_dir()
        .unwrap()
        .join(".project_manager_cli/project_index.toml");

    write_file(toml_str.as_bytes(), &project_index_file_path);
    println!("Project added to the index file")
}

fn write_file(content: &[u8], path: &PathBuf) {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
    }

    let mut file = fs::File::create(path).unwrap();

    file.write_all(content).unwrap();
}

fn project_exist(name: &str, path: &str) -> bool {
    let project_index = get_project_index();

    if project_index.projects.is_empty() {
        false
    } else {
        project_index
            .projects
            .iter()
            .any(|project| project.project_name == name || project.path == path)
    }
}

fn setup() {
    // check if directories are present
    let cli_project_location = dirs::home_dir()
        .unwrap()
        .join(".project_manager_cli/projects");
    if !cli_project_location.exists() {
        if let Some(parent) = cli_project_location.parent() {
            let _ = fs::create_dir_all(parent);
        }
    }
    // Check if the project_index file is present
    let project_index_file_location = dirs::home_dir()
        .unwrap()
        .join(".project_manager_cli/project_index.toml");

    if !project_index_file_location.exists() {
        File::create(project_index_file_location).expect("Could not create project index file");
    }
}
