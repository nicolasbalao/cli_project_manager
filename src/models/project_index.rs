use std::fs;

use super::project_config::ProjectMetaData;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectIndex {
    projects: Vec<ProjectMetaData>,
}

impl ProjectIndex {
    pub fn new() -> Self {
        ProjectIndex {
            projects: Vec::new(),
        }
    }

    pub fn load() -> Result<ProjectIndex, anyhow::Error> {
        let index_file_path = dirs::home_dir()
            .context("Failed to get home directory")?
            .join(".project_manager_cli/project_index.toml");

        if index_file_path.exists() {
            let toml_str =
                fs::read_to_string(index_file_path).context("Failed to read project index file")?;
            let project_index: ProjectIndex =
                toml::from_str(&toml_str).context("Failed to parse project index file")?;
            Ok(project_index)
        } else {
            Err(anyhow::anyhow!("Project index file not found"))
        }
    }

    pub fn load_or_new() -> ProjectIndex {
        match Self::load() {
            Ok(p_index) => p_index,
            Err(_) => Self::new(),
        }
    }

    pub fn add_project_and_save(
        &mut self,
        new_project: ProjectMetaData,
    ) -> Result<(), anyhow::Error> {
        if self.project_exists(&new_project.name, &new_project.path) {
            return Err(anyhow::anyhow!("Project already exists"));
        }

        self.projects.push(new_project);
        self.save()?;

        Ok(())
    }

    pub fn project_exists(&self, name: &str, path: &str) -> bool {
        self.projects
            .iter()
            .any(|p| p.name == name || p.path == path)
    }

    fn save(&self) -> Result<(), anyhow::Error> {
        let toml_str = toml::to_string(&self).context("Failed to serialize project index")?;
        let index_file_path = dirs::home_dir()
            .context("Failed to ger home directory")?
            .join(".project_manager_cli/project_index.toml");

        fs::create_dir_all(index_file_path.parent().unwrap())
            .context("Failed to create directory for project index file")?;

        fs::write(index_file_path, &toml_str).context("Failed to write project index file")?;
        println!("Project index file saved");
        Ok(())
    }
}
