use std::{
    fs::{self, File},
    io::Write,
};

use serial_test::serial;

#[test]
#[serial]
fn delete_project() -> Result<(), Box<dyn std::error::Error>> {
    // Mock config file and index file in tmp dir
    let config_path = tempfile::tempdir()
        .expect("failed to create config path tmp")
        .path()
        .join(".project_manager_cli");
    std::env::set_var("PROJECT_MANAGER_CLI_HOME", &config_path);
    let project_index_path = &config_path.join("project_index.toml");
    let project_config_file = &config_path.join("projects/TestProject.toml");

    let _ = fs::create_dir_all(&config_path);

    if let Some(parent) = project_config_file.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let _ = File::create(&project_config_file).expect("Failed to create project config file");

    let mut project_index_file =
        File::create(&project_index_path).expect("failed to create index file");

    let sample_data = r#"
            [[projects]]
            name = "TestProject"
            creation_date_utc = "2023-09-23T12:00:00Z"
            path = "some/path"

            [[projects]]
            name = "project_2"
            creation_date_utc = "2023-09-23T12:00:00Z"
            path = "some/path2"
        "#;

    let _ = &project_index_file
        .write_all(&sample_data.as_bytes())
        .expect("Failed to write index file");

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;
    cmd.arg("delete").arg("TestProject");

    cmd.assert().success();

    let config_file_exist = project_config_file.exists();
    assert!(!config_file_exist);

    let index_content = fs::read_to_string(&project_index_path).expect("Failed to read index file");

    assert!(!index_content.contains("TestProject"));

    Ok(())
}
