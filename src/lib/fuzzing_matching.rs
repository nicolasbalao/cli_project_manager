use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::lib::utils;

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let length_a = a.len();
    let length_b = b.len();

    let mut distances: Vec<Vec<usize>> = vec![vec![0; length_b + 1]; length_a + 1];

    for (i, distance) in distances.iter_mut().enumerate().take(length_a + 1) {
        distance[0] = i;
    }

    for j in 0..=length_b {
        distances[0][j] = j;
    }

    for i in 1..=length_a {
        for j in 1..=length_b {
            let cost = if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
                0
            } else {
                1
            };

            distances[i][j] = min(
                distances[i][j - 1] + 1,
                min(distances[i - 1][j] + 1, distances[i - 1][j - 1] + cost),
            );
        }
    }

    distances[length_a][length_b]
}

pub fn compute_matching_score(s1: &str, s2: &str) -> f64 {
    let s1 = utils::normalize_string(s1);

    let distance = levenshtein_distance(&s1, s2);

    let (len1, len2) = (s1.len(), s2.len());

    let mut score = 1.0 - (distance as f64 / max(len1, len2) as f64);

    if s1.starts_with(s2) {
        score += 0.3;
    }

    if distance <= 2 {
        score += 0.1
    }

    if s1.contains(s2) {
        score += 0.1
    }

    score.clamp(0.0, 1.0)
}

pub fn matching(source: Vec<&str>, pattern: &str) -> HashMap<u32, Vec<String>> {
    let mut result = HashMap::new();
    for sample in source {
        let score = compute_matching_score(sample, pattern);

        // TODO: Refactor this
        let score = (score * 100.00).round() as u32;

        result
            .entry(score)
            .or_insert_with(Vec::new)
            .push(sample.to_string());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        let test_cases = vec![
            ("kitten", "sitting", 3),
            ("flaw", "lawn", 2),
            ("gumbo", "gambol", 2),
            ("book", "back", 2),
            ("chat", "chats", 1),
            ("rosettacode", "raisethysword", 8),
            ("hello", "hello", 0),
            // Edge cases: empty strings
            ("", "", 0),
            ("", "abc", 3),
            ("abc", "", 3),
            // Similar but different lengths
            ("abcdef", "abc", 3),
            ("abc", "abcdef", 3),
            ("abcde", "abc", 2),
            // Single character differences
            ("a", "b", 1),
            ("ab", "ac", 1),
            ("abc", "axc", 1),
            ("abcdefg", "abcxefg", 1),
            // Completely different strings
            ("abc", "xyz", 3),
            ("abcdef", "uvwxyz", 6),
            ("abcdef", "ghijkl", 6),
            // Cases with repeated characters
            ("aaaaaa", "aa", 4),
            ("aaaa", "a", 3),
            ("ababab", "bababa", 2),
            // Complex cases with substitutions, insertions, and deletions
            ("sunday", "saturday", 3),
            ("intention", "execution", 5),
            ("distance", "difference", 5),
            ("algorithm", "altruistic", 6),
            ("levenshtein", "frankenstein", 6),
            // Case with spaces and special characters
            ("test case", "testcase", 1),
            ("rust-lang", "rusty-lang", 1),
            ("hello, world!", "hella, wurld?", 3),
            // Case sensitive checks
            ("abc", "ABC", 3),
            ("Rust", "rust", 1),
        ];

        for (source, pattern, distance) in test_cases {
            assert_eq!(
                levenshtein_distance(source, pattern),
                distance,
                "Failed on: {} -> {}",
                source,
                pattern
            )
        }
    }
}
