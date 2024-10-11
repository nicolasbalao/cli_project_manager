use std::{collections::HashMap, hash::Hash};

use regex::Regex;

pub fn normalize_string(input: &str) -> String {
    let re = Regex::new(r"[-_/\\ :]").expect("Failed to create regex exp");
    re.replace_all(input, "").to_string()
}

pub fn sort_hashmap_by_keys<K, V>(map: &HashMap<K, V>) -> Vec<(K, &V)>
where
    K: Eq + Hash + Ord + Copy, // K must implement Eq, Hash, and Ord traits
{
    // Convert the HashMap into a vector of tuples (key, value)
    let mut sorted: Vec<(K, &V)> = map.iter().map(|(k, v)| (*k, v)).collect();

    sorted.sort_by(|a, b| b.0.cmp(&a.0));

    sorted
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::lib::utils::{normalize_string, sort_hashmap_by_keys};

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

    #[test]
    fn test_sort_hashmap_by_keys_desc() {
        let mut sample = HashMap::new();

        sample.insert(10, "test");
        sample.insert(64, "test");
        sample.insert(123, "test");
        sample.insert(12, "test");

        let exepected = vec![(123, &"test"), (64, &"test"), (12, &"test"), (10, &"test")];

        assert_eq!(sort_hashmap_by_keys(&sample), exepected);
    }
    #[test]
    fn test_sort_hashmap_by_keys_asc() {
        let mut sample = HashMap::new();

        sample.insert(10, "test");
        sample.insert(64, "test");
        sample.insert(123, "test");
        sample.insert(12, "test");

        let exepected = vec![(10, &"test"), (12, &"test"), (64, &"test"), (123, &"test")];

        assert_eq!(sort_hashmap_by_keys(&sample), exepected);
    }
}
