use crate::models::{project_config::ProjectConfig, project_index::ProjectIndex};

pub fn execute(project_name: String, cmd: &String) {
    let project_index = ProjectIndex::load_or_new();

    let project_meta_data = match project_index.find_project_by_name(&project_name) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error getting project_meta_data: {:?}", e);
            std::process::exit(1);
        }
    };

    let project_config = ProjectConfig::load(&project_meta_data.name);

    println!("Project config: {:?}", project_config.commands);
}
