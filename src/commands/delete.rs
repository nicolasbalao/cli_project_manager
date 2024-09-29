use crate::models::project_index::ProjectIndex;

pub fn execute(project_name: &str) {
    let mut project_index = ProjectIndex::load_or_new();

    // TODO: Improve this
    match project_index.remove_project_by_name(project_name) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error while removing project{:?}", e);
            std::process::exit(0);
        }
    };

    println!("{project_name} removed !!");
}
