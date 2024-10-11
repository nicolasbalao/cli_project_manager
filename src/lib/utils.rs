use regex::Regex;

pub fn normalize_string(input: &str) -> String {
    let re = Regex::new(r"[-_/\\ :]").expect("Failed to create regex exp");
    re.replace_all(input, "").to_string()
}

#[cfg(test)]
mod test {
    use crate::lib::utils::normalize_string;

    #[test]
    fn test_normalize_string() {
        let samples = vec![
            ("cli-project_manager", "cliprojectmanager"),
            ("my_project-name/2024\\final", "myprojectname2024final"),
            ("project_1/test-case", "project1testcase"),
            ("----__//\\\\___", ""),
            ("  cli  / - project  _ manager  ", "cliprojectmanager"),
            (
                "C:/Users/Admin/cli_Project-Manager",
                "CUsersAdmincliProjectManager",
            ),
        ];

        for (input, expected) in samples {
            assert_eq!(
                normalize_string(input),
                expected,
                "Failed on {} -> {}",
                input,
                expected
            );
        }
    }
}
