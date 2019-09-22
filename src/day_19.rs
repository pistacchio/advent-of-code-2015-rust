use std::fs::read_to_string;
use regex::{Regex};
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

const INPUT_FILE: &str = "data/day_19.txt";
const TEST_REPLACEMENTS: &str = r#"H => HO
H => OH
O => HH"#;


fn parse_replacements(s: &str) -> Vec<(String, String)> {
    s.lines().map(|l| {
        let words: Vec<_> = l.split_whitespace().collect();
        (words[0].to_owned(), words[2].to_owned())
    })
        .collect()
}

fn apply_replacements(input: &str, replacements: &[(String, String)]) -> usize {
    let input = input.to_owned();
    let mut molecules: HashSet<String> = HashSet::new();

    for (replacement_s, replacement_r) in replacements {
        let local_input = input.clone();

        let regex = Regex::new(&format!("({})", replacement_s)).unwrap();

        for m in regex.find_iter(&local_input) {
            let pre_input = local_input[0..m.start()].to_string();
            let post_input = local_input[m.end()..].to_string();

            let molecule = format!("{}{}{}", pre_input, replacement_r, post_input);
            molecules.insert(molecule);
        }
    }

    molecules.len()
}

fn synthesize_molecule(input: &str, replacements: &[(String, String)], target: &str) -> i32 {
    let replacements: HashMap<String, String> = replacements.iter()
        .map(|r| (r.1.chars().rev().collect(), r.0.chars().rev().collect()))
        .collect();

    let mut count = 0;
    let mut actual_target = target.chars().rev().join("");
    let regex = Regex::new(&replacements.keys().join("|")).unwrap();

    while actual_target != input {
        actual_target = regex.replace(&actual_target, |captures: &regex::Captures| {
            replacements.get(&captures[0]).unwrap()
        }).to_string();

        count += 1;
    }

    count
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let split_regex = Regex::new(r#"\n\n"#).unwrap();
    let parts: Vec<_> = split_regex.split(&input).collect();

    let replacements = parse_replacements(parts[0]);
    apply_replacements(parts[1], &replacements).to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let split_regex = Regex::new(r#"\n\n"#).unwrap();
    let parts: Vec<_> = split_regex.split(&input).collect();

    let replacements = parse_replacements(parts[0]);
    synthesize_molecule(&"e", &replacements, parts[1]).to_string()
}

#[test]
fn test_run() {
    let replacements = parse_replacements(&TEST_REPLACEMENTS);
    assert_eq!(apply_replacements(&"HOH", &replacements), 4);
}
