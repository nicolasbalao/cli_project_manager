use std::{
    fmt, fs,
    path::{self},
};

use super::project_config::{ProjectConfig, ProjectMetaData};
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
        let config = crate::config::get_config().unwrap().read().unwrap();

        Self::load_from_path(&config.project_index_file)
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

        let config = crate::config::get_config().unwrap().read().unwrap();

        fs::create_dir_all(config.project_index_file.parent().unwrap())
            .context("Failed to create directory for project index file")?;

        fs::write(&config.project_index_file, &toml_str)
            .context("Failed to write project index file")?;
        println!("Project index file saved");
        Ok(())
    }

    pub fn find_project_by_name(
        &self,
        project_name: &str,
    ) -> Result<&ProjectMetaData, anyhow::Error> {
        self.projects
            .iter()
            .find(|project| project.name == project_name)
            .context(format!("Project {} not found", project_name))
    }

    pub fn remove_project_by_name(&mut self, project_name: &str) -> Result<(), anyhow::Error> {
        let project_index = self
            .projects
            .iter()
            .position(|p| p.name == project_name)
            .context("Project doesn't exist")?;

        let project_meta_data = self.projects.remove(project_index);

        let project_config = ProjectConfig::new(project_meta_data);
        project_config.remove()?;
        self.save()?;
        Ok(())
    }
}

impl fmt::Display for ProjectIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for project in &self.projects {
            writeln!(f, "{}", project)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use crate::models::project_config::ProjectMetaData;

    use super::*;

    #[test]
    fn test_new() {
        let project_index = ProjectIndex::new();
        assert!(project_index.projects.is_empty());
    }

    #[test]
    fn test_load_from_path_non_existent_file_index() {
        let temp_dir = tempfile::tempdir().unwrap();
        let index_file_path = temp_dir.into_path().join("non_existent_index_file.toml");

        let result = ProjectIndex::load_from_path(&index_file_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_path_existent_file_index() {
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

    #[test]
    fn test_project_exists() {
        let project_name = Some("project_test".to_string());
        let project_path = tempfile::tempdir().unwrap().into_path();

        let project_meta_data = ProjectMetaData::new(&project_path, project_name.clone()).unwrap();

        let mut project_index = ProjectIndex::new();
        project_index.projects.push(project_meta_data);

        assert!(project_index.project_exists("project_test", project_path.to_str().unwrap()));
    }

    // TODO:

    // Save

    // Find project
    #[test]
    fn test_find_project_by_name() {
        let tmp_dir = tempfile::tempdir().expect("Failed to create tmp dir");
        let project_meta_data =
            ProjectMetaData::new(tmp_dir.path(), Some("project_name".to_string()))
                .expect("Failed to create project meta data");

        let mut project_index = ProjectIndex::new();

        project_index.projects.push(project_meta_data);

        let project_meta_data_finded = project_index
            .find_project_by_name(&project_index.projects[0].name)
            .unwrap();

        assert_eq!(*project_meta_data_finded, project_index.projects[0]);
    }

    // Remove project
}
