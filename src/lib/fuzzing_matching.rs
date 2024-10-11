use std::cmp::min;

pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let length_a = a.len();
    let length_b = b.len();

    let mut distances: Vec<Vec<usize>> = vec![vec![0; length_b + 1]; length_a + 1];

    for i in 0..=length_a {
        distances[i][0] = i;
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

            display_levenshtein_distance(a, b, &distances);
        }
    }

    distances[length_a][length_b]
}

fn display_levenshtein_distance(a: &str, b: &str, distances: &Vec<Vec<usize>>) {
    let a = format!("X{a}");
    let b = format!("X{b}");

    print!("  ");

    for letter in b.chars() {
        print!("{} ", letter);
    }
    println!();

    for i in 1..=a.len() {
        print!("{} ", a.chars().nth(i - 1).unwrap());
        for j in 1..=b.len() {
            print!("{:?} ", distances[i - 1][j - 1]);
        }
        println!();
    }

    println!();
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
