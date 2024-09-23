use std::{
    fs,
    io::Write,
    path::{self},
};

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ProjectConfig {
    pub meta_data: ProjectMetaData,
}

impl ProjectConfig {
    pub fn new(meta_data: ProjectMetaData) -> Self {
        ProjectConfig { meta_data }
    }

    // TODO Add error handling
    pub fn save(&self) {
        let toml_str = toml::to_string(&self).unwrap();
        let config_file_path = dirs::home_dir().unwrap().join(format!(
            ".project_manager_cli/projects/{}.toml",
            self.meta_data.name
        ));

        let mut config_file =
            fs::File::create(config_file_path).expect("Trouble to create project config file");

        config_file
            .write_all(&toml_str.as_bytes())
            .expect("Failed to write the config file");

        println!(
            "Project config file created at : ~/.project_manager_cli/projects/{}.toml",
            self.meta_data.name
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMetaData {
    pub name: String,
    pub creation_date_utc: String,
    pub path: String,
}

impl ProjectMetaData {
    pub fn new(path: &path::PathBuf, project_name: Option<String>) -> Self {
        let path = path.canonicalize().unwrap();
        let canonical_path = path.canonicalize().unwrap().to_string_lossy().to_string();
        let name = project_name.unwrap_or_else(|| {
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap_or("Unamed project")
                .to_string()
        });

        ProjectMetaData {
            name,
            path: canonical_path,
            creation_date_utc: Utc::now().to_string(),
        }
    }
}
