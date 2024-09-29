use std::process::Command;

use crate::models::project_index::ProjectIndex;

pub fn execute(project_name: String) {
    let project_index = ProjectIndex::load_or_new();

    if project_index.projects.is_empty() {
        println!("List of projects is empty!");
        return;
    }

    // Search project in the project index
    let project_meta_data = match project_index.find_project(&project_name) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error finding project form project index {:?}", e);
            std::process::exit(1)
        }
    };

    let config = crate::config::get_config().unwrap().read().unwrap();

    let editor_command = &config.config.editor_cmd;
    // Open the editor in the directory
    Command::new(editor_command)
        .arg("-n")
        .arg(&project_meta_data.path)
        .spawn()
        .expect("Failed to open vscode");
}
