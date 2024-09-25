use core::str;
use std::{fs, thread, time::Duration};

use serial_test::serial;

/// 1. Test normal behaviour command
///    add <path> --name <project-name>
#[test]
#[serial]
fn add_new_project_with_name() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir().expect("Failed to create tmp dir");
    let temp_project_dir = tempfile::tempdir().expect("Failed to create tmp dir");

    let config_path = temp_dir.path().join(".project_manager_cli");
    std::env::set_var(
        "PROJECT_MANAGER_CLI_HOME",
        &config_path.as_path().to_str().unwrap(),
    );

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;

    cmd.arg("add")
        .arg(temp_project_dir.path())
        .arg("--name")
        .arg("test_project");

    cmd.assert().success();

    // Verify that the project index file has been created
    let project_index_path = config_path.join("project_index.toml");
    assert!(project_index_path.exists());

    let project_index_file = fs::read_to_string(project_index_path)?;
    assert!(project_index_file.contains("test_project"));

    // Check that the project configuration file exists
    let project_config_path = config_path.join("projects/test_project.toml");
    assert!(project_config_path.exists());

    // Step 5: Clean up environment variable (optional)
    std::env::remove_var("PROJECT_MANAGER_CLI_HOME");

    Ok(())
}

/// 2. Test without the name
///    add <path>
#[test]
#[serial]
fn add_new_project_without_name() -> Result<(), Box<dyn std::error::Error>> {
    let project_name = "project_without_name";
    let temp_dir = tempfile::tempdir().expect("Create temp dir");
    let temp_project_dir = tempfile::tempdir()
        .expect("Create temp dir")
        .path()
        .join(project_name);

    // Create the project_dir
    let _ =
        fs::create_dir_all(&temp_project_dir).expect("Failed to create dir for the tmp project");

    let config_path = temp_dir.path().join(".project_manager_cli");
    std::env::set_var(
        "PROJECT_MANAGER_CLI_HOME",
        &config_path.as_path().to_str().unwrap(),
    );

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;

    cmd.arg("add").arg(&temp_project_dir);

    cmd.assert().success();

    // Verify that the project index file has been created
    let project_index_path = config_path.join("project_index.toml");
    wait_for_condition(|| project_index_path.exists());
    assert!(project_index_path.exists());

    let project_index_file = fs::read_to_string(project_index_path)?;
    assert!(project_index_file.contains(project_name));

    // Check that the project configuration file exists
    let project_config_path = config_path.join(format!("projects/{}.toml", project_name));
    assert!(project_config_path.exists());

    // Step 5: Clean up environment variable (optional)
    std::env::remove_var("PROJECT_MANAGER_CLI_HOME");

    Ok(())
}

// 3. Test with bad path
//    add <path>

#[test]
#[serial]
fn add_new_project_with_bad_path() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir().expect("Create temp dir");
    let temp_project_dir = tempfile::tempdir().unwrap().path().join("some/bad/path");

    let config_path = temp_dir.path().join(".project_manager_cli");
    std::env::set_var(
        "PROJECT_MANAGER_CLI_HOME",
        &config_path.as_path().to_str().unwrap(),
    );

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;

    let output = cmd
        .arg("add")
        .arg(temp_project_dir)
        .output()
        .expect("Failed to run command");

    assert!(!output.status.success());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("Failed to canonicalize path"));

    // Step 5: Clean up environment variable (optional)
    std::env::remove_var("PROJECT_MANAGER_CLI_HOME");

    Ok(())
}

// 4. Test with project with an existing name
//    add <path> --name <existing-name>

#[test]
#[serial]
fn create_a_project_with_an_existing_name() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir().expect("Create temp dir");
    let temp_project_dir = tempfile::tempdir()
        .expect("Create temp dir")
        .path()
        .join("project_folder");

    let config_path = temp_dir.path().join(".project_manager_cli");
    std::env::set_var(
        "PROJECT_MANAGER_CLI_HOME",
        &config_path.as_path().to_str().unwrap(),
    );

    let _ = fs::create_dir_all(&temp_project_dir)
        .expect("Failed to create directory for the tmp project");

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;

    let output = cmd
        .arg("add")
        .arg(&temp_project_dir)
        .arg("--name")
        .arg("project_name")
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());

    let new_temp_project_dir = tempfile::tempdir()
        .expect("Create temp dir")
        .path()
        .join("project_folder_2");

    let _ =
        fs::create_dir_all(&new_temp_project_dir).expect("Failed to create new project tmp dir");
    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;
    let output = cmd
        .arg("add")
        .arg(&new_temp_project_dir)
        .arg("--name")
        .arg("project_name")
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).expect("Failed to get the stdout");

    assert!(stdout.contains("Project already exist"));

    Ok(())
}

// 5. Test with project with an existing path
//    add <path>
#[test]
#[serial]
fn create_a_project_with_an_existing_path() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir().expect("Create temp dir");
    let temp_project_dir = tempfile::tempdir()
        .expect("Create temp dir")
        .path()
        .join("project_folder");

    let config_path = temp_dir.path().join(".project_manager_cli");
    std::env::set_var(
        "PROJECT_MANAGER_CLI_HOME",
        &config_path.as_path().to_str().unwrap(),
    );

    let _ = fs::create_dir_all(&temp_project_dir).expect("Failed to create the tmp project dir");

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;

    let output = cmd
        .arg("add")
        .arg(&temp_project_dir)
        .arg("--name")
        .arg("project_1")
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());

    let mut cmd = assert_cmd::Command::cargo_bin("cli_project_manager")?;
    let output = cmd
        .arg("add")
        .arg(&temp_project_dir)
        .arg("--name")
        .arg("project_2")
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
    let stdout = std::str::from_utf8(&output.stdout).expect("Failed to get the stdout");

    assert!(stdout.contains("Project already exist"));

    Ok(())
}

// Utils
fn wait_for_condition<F>(condition: F)
where
    F: Fn() -> bool,
{
    let timeout = Duration::from_secs(5);
    let start = std::time::Instant::now();

    while !condition() {
        if start.elapsed() > timeout {
            panic!("Condition was not met in time");
        }

        thread::sleep(Duration::from_millis(10));
    }
}
