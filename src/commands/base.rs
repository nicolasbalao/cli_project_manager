use std::{io::stdin, process::Command};

use cli_project_manager::lib;

use crate::models::{project_config::ProjectMetaData, project_index::ProjectIndex};

pub fn execute(project_name: String) {
    let project_index = ProjectIndex::load_or_new();

    if project_index.projects.is_empty() {
        println!("List of projects is empty!");
        return;
    }

    // Search project in the project index
    // let project_meta_data = match project_index.find_project_by_name(&project_name) {
    //     Ok(p) => p,
    //     Err(e) => {
    //         eprintln!("Error finding project form project index {:?}", e);
    //         std::process::exit(1)
    //     }
    // };

    let project_names = project_index
        .projects
        .iter()
        .map(|p| p.name.as_str())
        .collect();

    let project_names_fuzzed = lib::fuzzing_matching::matching(project_names, &project_name);
    let project_names_fuzzed = lib::utils::sort_hashmap_by_keys(&project_names_fuzzed);

    // REFACTOR
    let project_meta_data =
        if project_names_fuzzed[0].0 > 60 && project_names_fuzzed[0].1.len() == 1 {
            project_index
                .find_project_by_name(project_names_fuzzed[0].1[0].as_str())
                .unwrap()
        } else {
            let mut index = String::new();
            let stdin = stdin();
            for (i, (_, project_name)) in project_names_fuzzed.iter().enumerate() {
                println!("{} : {}", i, project_name[0]);
            }
            stdin.read_line(&mut index).expect("Failed to read stdin");

            let index = index.trim();
            let index: usize = index.parse().unwrap();

            let (_, project_names) = project_names_fuzzed.get(index).unwrap();
            project_index
                .find_project_by_name(&project_names[0])
                .expect("Failed to find project metadata")
        };

    // Lunch vscode with code .
    spawn_editor(project_meta_data);

    let mut shell = Command::new("zsh")
        .env("PROJECT_NAME", &project_meta_data.name)
        .current_dir(&project_meta_data.path)
        .spawn()
        .expect("Failed to spawn shell");

    shell.wait().expect("Failed to wait shell processus");
}

fn spawn_editor(project_meta_data: &ProjectMetaData) {
    let _ = Command::new("code")
        .env("PROJECT_NAME", &project_meta_data.name)
        .arg(".")
        .current_dir(&project_meta_data.path)
        .spawn()
        .expect("Failed to spawn code");
}
