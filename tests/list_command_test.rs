use core::str;
use std::{
    fs::{self, File},
    io::Write,
};

use serial_test::serial;
#[test]
#[serial]
fn listing_project() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = tempfile::tempdir()
        .expect("failed to create config path tmp")
        .path()
        .join(".project_manager_cli");
    std::env::set_var("PROJECT_MANAGER_CLI_HOME", &config_path);
    let project_index_path = &config_path.join("project_index.toml");

    let _ = fs::create_dir_all(&config_path);

    let mut project_index_file =
        File::create(&project_index_path).expect("failed to create index file");

    let sample_data = r#"
            [[projects]]
            name = "TestProject"
            creation_date_utc = "2023-09-23T12:00:00Z"
            path = "some/path"

            [[projects]]
            name = "TestProject2"
            creation_date_utc = "2023-09-23T12:00:00Z"
            path = "some/path2"
        "#;

    let _ = &project_index_file
        .write_all(&sample_data.as_bytes())
        .expect("Failed to write index file");

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;
    let output = cmd.arg("list").output().expect("failed to run command");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("TestProject"));
    assert!(stdout.contains("TestProject2"));
    Ok(())
}

// No project

#[test]
#[serial]
fn listing_project_with_empty_index() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = tempfile::tempdir()
        .expect("failed to create config path tmp")
        .path()
        .join(".project_manager_cli");
    std::env::set_var(
        "PROJECT_MANAGER_CLI_HOME",
        &config_path.to_string_lossy().to_string(),
    );
    let project_index_path = &config_path.join("project_index.toml");

    let _ = fs::create_dir_all(&config_path);

    let _ = File::create(&project_index_path).expect("failed to create index file");

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;
    let output = cmd.arg("list").output().expect("failed to run command");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("No project found"));
    Ok(())
}
