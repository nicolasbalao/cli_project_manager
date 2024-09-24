use std::{
    fs,
    path::{self},
};

use super::project_config::ProjectMetaData;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectIndex {
    pub projects: Vec<ProjectMetaData>,
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

        Self::load_from_path(&index_file_path)
    }

    fn load_from_path(index_file_path: &path::PathBuf) -> Result<ProjectIndex, anyhow::Error> {
        if index_file_path.exists() {
            let toml_str =
                fs::read_to_string(index_file_path).context("Failed to read project index")?;
            let projet_index =
                toml::from_str(&toml_str).context("Failed to parse project index file")?;
            Ok(projet_index)
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

#[cfg(test)]
mod test {
    use std::io::Write;

    use crate::models::project_config::ProjectMetaData;

    use super::*;

    #[test]
    fn project_index_creation() {
        let project_index = ProjectIndex::new();
        assert!(project_index.projects.is_empty());
    }

    #[test]
    fn project_exists() {
        let project_name = Some("project_test".to_string());
        let project_path = tempfile::tempdir().unwrap().into_path();

        let project_meta_data = ProjectMetaData::new(&project_path, project_name.clone()).unwrap();

        let mut project_index = ProjectIndex::new();
        project_index.projects.push(project_meta_data);

        assert!(project_index.project_exists("project_test", project_path.to_str().unwrap()));
    }

    #[test]
    fn load_non_existent_file_index() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_file_path = temp_dir.into_path().join("non_existent_index_file.toml");

        let result = ProjectIndex::load_from_path(&index_file_path);

        assert!(result.is_err());
    }

    #[test]
    fn load_existent_file_index() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_file_path = temp_dir.path().join("project_index.toml");

        let mut file = fs::File::create(&index_file_path).unwrap();

        let sample_data = r#"
            [[projects]]
            name = "TestProject"
            creation_date_utc = "2023-09-23T12:00:00Z"
            path = "some/path"
        "#;

        file.write_all(&sample_data.as_bytes()).unwrap();

        let result = ProjectIndex::load_from_path(&index_file_path);

        assert!(result.is_ok());

        let index = result.unwrap();

        assert_eq!(index.projects.len(), 1);
        assert_eq!(index.projects[0].name, "TestProject");
    }
}
