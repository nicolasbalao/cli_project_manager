use crate::models::project_index::ProjectIndex;

pub fn execute(project_name: String, cmd: &String) {
    let project_index = ProjectIndex::load_or_new();

    let project_meta_data = match project_index.find_project_by_name(&project_name) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error getting project_meta_data: {:?}", e);
            std::process::exit(1);
        }
    };

    println!("Project meta data: {:?}", project_meta_data);
}
