use std::{
    fs,
    io::Write,
    path::{self},
};

use anyhow::{Context, Ok};
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
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let toml_str = toml::to_string(&self).context("Failed to serialize project config")?;
        let config_file_path = dirs::home_dir()
            .context("Failed to get home directory")?
            .join(format!(
                ".project_manager_cli/projects/{}.toml",
                self.meta_data.name
            ));

        let mut config_file =
            fs::File::create(config_file_path).context("Failed creating  project config file")?;

        config_file
            .write_all(&toml_str.as_bytes())
            .context("Failed to write the config file")?;

        println!(
            "Project config file created at : ~/.project_manager_cli/projects/{}.toml",
            self.meta_data.name
        );

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMetaData {
    pub name: String,
    pub creation_date_utc: String,
    pub path: String,
}

impl ProjectMetaData {
    pub fn new(path: &path::PathBuf, project_name: Option<String>) -> Result<Self, anyhow::Error> {
        let path = path.canonicalize().context("Failed to canonicalize path")?;
        let canonical_path = path.to_string_lossy().to_string();
        let name = project_name.unwrap_or_else(|| {
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap_or("Unamed project")
                .to_string()
        });

        Ok(ProjectMetaData {
            name,
            path: canonical_path,
            creation_date_utc: Utc::now().to_string(),
        })
    }
}
