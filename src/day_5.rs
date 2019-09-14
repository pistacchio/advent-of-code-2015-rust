use itertools::Itertools;
use std::fs::read_to_string;

const INPUT_FILE: &str = "data/day_5.txt";

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const DISALLOWED_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

trait Nice {
    fn is_nice(&self) -> bool;
    fn is_nicer(&self) -> bool;
}

impl Nice for String {
    fn is_nice(&self) -> bool {
        self.chars().filter(|c| VOWELS.contains(c)).count() >= 3
            && self.chars().group_by(|&c| c).into_iter().any(|(_, vals)| vals.count() > 1)
            && !DISALLOWED_STRINGS.iter().any(|ds| self.contains(ds))
    }

    fn is_nicer(&self) -> bool {
        self.chars()
            .enumerate()
            .zip(self.chars().enumerate().skip(1))
            .sorted_by_key(|g| format!("{}{}", (g.0).1, (g.1).1)) // each char is now a tuple with the char and the pos
            .group_by(|g| format!("{}{}", (g.0).1, (g.1).1)) // group by char pairs
            .into_iter()
            .map(|(key, val)| (key, val.collect::<Vec<_>>())) // collect the group values
            .filter(|(_, val)| val.iter().count() > 1) // exclude not repeated char groups
            .filter(|(_, val)| val.len() != 2 || (val[0].1).0 != (val[1].0).0) // exclude consecutive repetitions like "aaa"
            .any(|(_, vals)| vals.len() > 1) // a valid word must have at least two non consecutive repetitions

            && self.chars()
            .zip(self.chars().skip(1))
            .zip(self.chars().skip(2))
            .any(|g| (g.0).0 == g.1)
    }
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    input.lines()
        .filter(|s| s.to_string().is_nice())
        .count()
        .to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    input.lines()
        .filter(|s| s.to_string().is_nicer())
        .count()
        .to_string()
}

#[test]
fn test_run() {
    assert!("ugknbfddgicrmopn".to_string().is_nice());
    assert!("aaa".to_string().is_nice());
    assert!(!"jchzalrnumimnmhp".to_string().is_nice());
    assert!(!"haegwjzuvuyypxyu".to_string().is_nice());
    assert!(!"dvszwmarrgswjxmb".to_string().is_nice());
}

#[test]
fn test_run_pt2() {
    assert!("xyxy".to_string().is_nicer());
    assert!(!"aaa".to_string().is_nicer());
    assert!(!"aabcdefgaa".to_string().is_nicer());
    assert!("qjhvhtzxzqqjkmpb".to_string().is_nicer());
    assert!("xxyxx".to_string().is_nicer());
    assert!(!"uurcxstgmygtbstg".to_string().is_nicer());
    assert!(!"ieodomkazucvgmuy".to_string().is_nicer());
    assert!("rxexcbwhiywwwwnu".to_string().is_nicer());
}
