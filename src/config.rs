use anyhow::Context;
use dirs;
use once_cell::sync::OnceCell;
use std::fs::File;
use std::path;
use std::sync::RwLock;
use std::{fs, io, path::PathBuf};

pub struct Config {
    pub base_dir: path::PathBuf,
    pub project_index_file: path::PathBuf,
}

impl Config {
    pub fn new(base_dir: Option<PathBuf>) -> Result<Self, anyhow::Error> {
        let base_dir =
            base_dir.unwrap_or_else(|| dirs::home_dir().unwrap().join(".project_manager_cli"));

        ensure_directory_exists(&base_dir).context("Failed to ensure project directory exists")?;

        let project_index_file = base_dir.join("project_index.toml");

        ensure_file_exists(&project_index_file).context("Failed to create project index file")?;

        Ok(Config {
            base_dir,
            project_index_file,
        })
    }
}

static CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();

// Function to initialize the global config
pub fn init_config() -> Result<(), anyhow::Error> {
    let config = Config::new(None)?;
    CONFIG
        .set(RwLock::new(config))
        .map_err(|_| anyhow::anyhow!("Config was already initialized"))?;
    Ok(())
}

// Function to access the global config
pub fn get_config() -> Option<&'static RwLock<Config>> {
    CONFIG.get()
}

/// TODO: Move this utils module or other place
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
