use std::process::Command;

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

    let cmd_exist = project_config.commands.contains_key(cmd);

    if !cmd_exist {
        eprintln!("Sorry the command is not found");
        std::process::exit(1);
    }

    if let Some(shell_cmd) = project_config.commands.get(cmd) {
        let (shell_cmd, args) = prepare_command_for_execution(shell_cmd);
        println!("Command: {} Args: {:?}", shell_cmd, args);
        let mut cmd_process = Command::new(shell_cmd)
            .args(args)
            .current_dir(project_config.meta_data.path)
            .spawn()
            .expect("Failed to spawn the processus");
        let _ = cmd_process.wait();
    } else {
        eprintln!("Sorry the command is not found");
        std::process::exit(1);
    }
}

fn prepare_command_for_execution(cmd: &str) -> (&str, Vec<&str>) {
    let mut cmd_parts = cmd.split_whitespace();

    let command = cmd_parts.next().expect("Failed to parse command");
    let args: Vec<&str> = cmd_parts.collect();

    (command, args)
}
