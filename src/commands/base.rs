use std::{io::stdin, process::Command};

use cli_project_manager::lib;

use crate::models::{project_config::ProjectMetaData, project_index::ProjectIndex};

pub fn execute(project_name: String) {
    let project_index = ProjectIndex::load_or_new();

    if project_index.projects.is_empty() {
        println!("List of projects is empty!");
        return;
    }

    let project_meta_data = find_or_fuzzing_match_project(&project_index, &project_name);

    // Lunch vscode with code .
    spawn_editor(project_meta_data);

    let mut shell = Command::new("zsh")
        .env("PROJECT_NAME", &project_meta_data.name)
        .current_dir(&project_meta_data.path)
        .spawn()
        .expect("Failed to spawn shell");

    shell.wait().expect("Failed to wait shell processus");
}

fn find_or_fuzzing_match_project<'a>(
    project_index: &'a ProjectIndex,
    project_name: &'a str,
) -> &'a ProjectMetaData {
    // 1. Check for an exact match first
    if let Ok(project_meta_data) = project_index.find_project_by_name(project_name) {
        return project_meta_data;
    }

    // 2. Perform fuzzy matching if no exact match is found

    let project_names: Vec<&str> = project_index
        .projects
        .iter()
        .map(|p| p.name.as_str())
        .collect();

    let fuzzed_matches = lib::fuzzing_matching::matching(project_names, project_name);
    let mut sorted_matches = lib::utils::sort_hashmap_by_keys(&fuzzed_matches);

    // The goal is to create an array that contains elements with a score difference of 20 or less.
    // This ensures that elements in the array are close in score, preventing the user from having
    // to choose between unrelated or vastly different options.

    let filtered_matches = filter_by_score_gap(&mut sorted_matches, 20);
    println!("Filtered matches: {:?}", filtered_matches);

    if filtered_matches.len() == 1 {
        let matched_name = &filtered_matches[0].1[0];
        return project_index
            .find_project_by_name(matched_name)
            .unwrap_or_else(|e| {
                eprintln!(
                    "Error finding project by name after fuzzing matching: {:?}",
                    e
                );
                std::process::exit(1);
            });
    }

    // 4. Handle mutliple fuzzy matches or low confidence matches by prompting the user
    let project_name = prompt_user_for_project_selection(&filtered_matches);

    let project_meta_data = project_index
        .find_project_by_name(&project_name)
        .expect("Failed to find project");

    project_meta_data
}

fn filter_by_score_gap<'a>(
    sorted_matches: &mut [(u32, &'a Vec<String>)],
    gap: u32,
) -> Vec<(u32, &'a Vec<String>)> {
    let mut filtered_matches = vec![sorted_matches[0]];

    if sorted_matches[0].0 - sorted_matches[1].0 > gap {
        return filtered_matches;
    }

    let tmp: Vec<(u32, &Vec<String>)> = sorted_matches
        .windows(2)
        .filter(|window| window[0].0 - window[1].0 <= gap)
        .map(|win| win[1])
        .collect();

    filtered_matches.extend(tmp);
    filtered_matches
}

fn prompt_user_for_project_selection(sorted_matches: &[(u32, &Vec<String>)]) -> String {
    println!("Multiple projects matched. Please select one:");

    // Display the list of projects to the user
    for (i, (_, project_names)) in sorted_matches.iter().enumerate() {
        println!("{}: {}", i, project_names[0]);
    }

    // Read user input and parse the selected index
    let mut index_input = String::new();
    let stdin = stdin();
    stdin
        .read_line(&mut index_input)
        .expect("Failed to read input");
    let index: usize = index_input
        .trim()
        .parse()
        .expect("Invalid input. Please enter a number.");

    // Get the selected project name and return its metadata
    let selected_project_name = &sorted_matches[index].1[0];

    String::from(selected_project_name)
}

fn spawn_editor(project_meta_data: &ProjectMetaData) {
    let _ = Command::new("code")
        .env("PROJECT_NAME", &project_meta_data.name)
        .arg(".")
        .current_dir(&project_meta_data.path)
        .spawn()
        .expect("Failed to spawn code");
}

#[cfg(test)]
mod test {

    use std::vec;

    use super::*;
    #[test]
    fn test_filter_by_score_gap() {
        let a = vec!["a".to_string()];
        let b = vec!["b".to_string()];
        let c = vec!["c".to_string()];
        let d = vec!["d".to_string()];
        let all_pass_match: Vec<(u32, &Vec<String>)> =
            vec![(100, &a), (90, &b), (80, &c), (70, &d)];
        let all_pass_expected = vec![(100, &a), (90, &b), (80, &c), (70, &d)];

        let only_first: Vec<(u32, &Vec<String>)> = vec![(100, &a), (50, &b), (30, &c), (20, &d)];
        let only_first_expected = vec![(100, &a)];

        let first_three: Vec<(u32, &Vec<String>)> = vec![(100, &a), (90, &b), (80, &c), (10, &d)];
        let first_three_expected = vec![(100, &a), (90, &b), (80, &c)];

        let samples = vec![
            (all_pass_match, all_pass_expected),
            (only_first, only_first_expected),
            (first_three, first_three_expected),
        ];

        for (mut input, expected) in samples {
            let result = filter_by_score_gap(&mut input, 20);
            assert_eq!(
                result, expected,
                "Failed on input: {:?} -> {:?}",
                input, expected
            );
        }
    }
}
