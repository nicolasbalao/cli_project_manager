use std::path;

use crate::models::{
    project_config::{ProjectConfig, ProjectMetaData},
    project_index::ProjectIndex,
};

pub fn execute(path: &path::PathBuf, project_name: &Option<String>) {
    let project_meta_data = ProjectMetaData::new(path, project_name.clone());

    let project_config = ProjectConfig::new(project_meta_data);

    let mut project_index = ProjectIndex::load_or_new();

    if project_index.project_exists(
        &project_config.meta_data.name,
        &project_config.meta_data.path,
    ) {
        println!("Project already exist");
        std::process::exit(0);
    }

    project_config.save();
    project_index
        .add_project_and_save(project_config.meta_data)
        .expect("Can't add project");
}
