use std::fs;

use super::project_config::ProjectMetaData;
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

    pub fn load() -> Result<ProjectIndex, std::io::Error> {
        let index_file_path = dirs::home_dir()
            .unwrap()
            .join(".project_manager_cli/project_index.toml");

        if index_file_path.exists() {
            let toml_str = fs::read_to_string(index_file_path)?;
            let project_index: ProjectIndex = toml::from_str(&toml_str)?;
            Ok(project_index)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Project index file not found",
            ))
        }
    }

    pub fn load_or_new() -> Self {
        match Self::load() {
            Ok(p_index) => p_index,
            Err(_) => Self::new(),
        }
    }

    pub fn add_project_and_save(&mut self, new_project: ProjectMetaData) -> Result<(), String> {
        if self.project_exists(&new_project.name, &new_project.path) {
            return Err("Project already exist".to_string());
        }

        self.projects.push(new_project);
        self.save().unwrap();

        Ok(())
    }

    pub fn project_exists(&self, name: &str, path: &str) -> bool {
        self.projects
            .iter()
            .any(|p| p.name == name || p.path == path)
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let toml_str = toml::to_string(&self).unwrap();
        let index_file_path = dirs::home_dir()
            .unwrap()
            .join(".project_manager_cli/project_index.toml");

        fs::create_dir_all(index_file_path.parent().unwrap())?;
        fs::write(index_file_path, &toml_str)?;
        println!("Project index file saved");
        Ok(())
    }
}
