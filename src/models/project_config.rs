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
        let project_config_file_path = project_config_path(&self.meta_data.name);

        println!(
            "Path file: {:?}",
            project_config_file_path.parent().unwrap().exists()
        );
        let mut config_file = fs::File::create(project_config_file_path)
            .context("Failed creating  project config file")?;

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

fn project_config_path(project_name: &str) -> path::PathBuf {
    let config = crate::config::get_config().unwrap().read().unwrap();
    config
        .base_dir
        .join(format!("projects/{}.toml", project_name))
        .clone()
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

#[cfg(test)]
mod test {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn create_project_meta_data() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_path = temp_dir.into_path();

        let project_meta_data =
            ProjectMetaData::new(&project_path, Some("project_name".to_string())).unwrap();

        assert_eq!(project_meta_data.name, "project_name");
        assert_eq!(
            project_meta_data.path,
            project_path
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .to_string()
        );
    }

    #[test]
    fn create_project_meta_data_without_name() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_path = temp_dir.into_path();
        let project_name = project_path
            .into_iter()
            .last()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let project_meta_data = ProjectMetaData::new(&project_path, None).unwrap();

        assert_eq!(project_meta_data.name, project_name);
        assert_eq!(
            project_meta_data.path,
            project_path
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .to_string()
        );
    }

    #[test]
    #[should_panic]
    fn create_project_meta_data_with_invalide_path() {
        let invalide_path =
            path::PathBuf::new().join("/tmp/jflsdjflksdjflkjslfjlsdfbdshjkgvbnjkhcvfh");

        let _ = ProjectMetaData::new(&invalide_path, None).unwrap();
    }

    fn mock_write_file(content: &[u8]) -> Result<(), anyhow::Error> {
        let mut buffer = Cursor::new(Vec::new());
        buffer.write_all(content)?;
        assert!(!buffer.get_ref().is_empty());
        Ok(())
    }

    #[test]
    fn create_project_and_save() {
        let project_name = Some("test_project".to_string());
        let project_path = tempfile::tempdir().unwrap().into_path();
        let project_meta_data = ProjectMetaData::new(&project_path, project_name.clone())
            .expect("Failed to create project metadata");

        let project_config = ProjectConfig::new(project_meta_data);

        let toml_str =
            toml::to_string(&project_config).expect("Failed to serialize project config");

        let result = mock_write_file(toml_str.as_bytes());

        assert!(result.is_ok());
        println!("Test passed: Porject config serialized and mock savec correctly");
    }

    #[test]
    fn test_project_config_file_path() {
        crate::config::init_config().unwrap();
        let project_name = "test_project";
        let project_config_path = project_config_path(&project_name);

        let config = crate::config::get_config().unwrap().read().unwrap();

        assert_eq!(
            config
                .base_dir
                .join(format!("projects/{project_name}.toml"))
                .to_string_lossy()
                .to_string(),
            project_config_path.to_string_lossy().to_string()
        )
    }
}
