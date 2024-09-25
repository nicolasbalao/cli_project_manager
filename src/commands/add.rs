use std::path::Path;

use crate::models::{
    project_config::{ProjectConfig, ProjectMetaData},
    project_index::ProjectIndex,
};

pub fn execute(path: &Path, project_name: &Option<String>) {
    let project_meta_data = match ProjectMetaData::new(path, project_name.clone()) {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("Error creating project metadata: {:?}", e);
            std::process::exit(1)
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
        std::process::exit(1)
    };
    if let Err(e) = project_index.add_project_and_save(project_config.meta_data) {
        eprintln!("Error adding project to index: {:?}", e);
        std::process::exit(1)
    }
}
