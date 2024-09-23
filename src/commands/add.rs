use std::path;

use crate::models::{
    project_config::{ProjectConfig, ProjectMetaData},
    project_index::ProjectIndex,
};

pub fn execute(path: &path::PathBuf, project_name: &Option<String>) {
    let project_meta_data = match ProjectMetaData::new(path, project_name.clone()) {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("Error creating project metadata: {:?}", e);
            return;
        }
    };

    let project_config = ProjectConfig::new(project_meta_data);

    let mut project_index = ProjectIndex::load_or_new();

    if project_index.project_exists(
        &project_config.meta_data.name,
        &project_config.meta_data.path,
    ) {
        println!("Project already exist");
        std::process::exit(0);
    }

    if let Err(e) = project_config.save() {
        eprintln!("Error saving project config: {:?}", e);
        return;
    };
    if let Err(e) = project_index.add_project_and_save(project_config.meta_data) {
        eprintln!("Error adding project to index: {:?}", e);
        return;
    }
}
