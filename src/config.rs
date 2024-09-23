use dirs;
use std::fs::File;
use std::{fs, io, path::PathBuf};

/// Sets up the necessary directory structure and the project index file.
pub fn setup_environment() -> io::Result<()> {
    let project_dir = get_project_dir();
    let project_index_file = get_project_index_path();

    // Ensure the project directory exists
    ensure_directory_exists(&project_dir)?;

    // Ensure the project index file exists
    ensure_file_exists(&project_index_file)?;

    Ok(())
}

/// Returns the path to the project directory
fn get_project_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".project_manager_cli/projects")
}

/// Returns the path to the project index file
fn get_project_index_path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".project_manager_cli/project_index.toml")
}

/// Ensures that a directory exists, creating it if necessary.
fn ensure_directory_exists(dir: &PathBuf) -> io::Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

/// Ensures that a file exists, creating it if necessary.
fn ensure_file_exists(file: &PathBuf) -> io::Result<()> {
    if !file.exists() {
        File::create(file)?;
    }
    Ok(())
}
