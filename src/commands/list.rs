use std::io::{self, Write};

use crate::models::project_index::ProjectIndex;

pub fn execute() {
    let project_index = ProjectIndex::load_or_new();

    if project_index.projects.is_empty() {
        println!("No projects found");
        return;
    }

    let mut stdout = io::stdout();

    writeln!(stdout, "----- Projects -----").expect("Failed to write in stdout");
    writeln!(stdout, "{}", project_index).expect("Failed to write to the stdout");
}
