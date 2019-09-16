use regex::{Regex};
use std::fs::read_to_string;

const INPUT_FILE: &str = "data/day_8.txt";

lazy_static! {
    static ref REPLACE_QUOTE_REGEX: Regex = Regex::new(r#"\\""#).unwrap();
    static ref REPLACE_SLASHES_REGEX: Regex = Regex::new(r"\\\\").unwrap();
    static ref REPLACE_CHAR_REGEX: Regex = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
}

fn count_actual_chars(s: &str) -> i32 {
    let s = REPLACE_QUOTE_REGEX.replace_all(&s, r#"""#);
    let s = REPLACE_SLASHES_REGEX.replace_all(&s, r"\");
    let s = REPLACE_CHAR_REGEX.replace_all(&s, "x");

    s.chars().count() as i32 - 2
}

fn re_encode_and_count(s: &str) -> i32 {
    let s =  s.replace(r#"\"#, r#"\\"#);
    let s =  s.replace(r#"""#, r#"\""#);

    s.chars().count() as i32 + 2
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    let actual_chars: i32 = input.lines().map(|l| count_actual_chars(l)).sum();
    let raw_chars: i32 = input.lines().map(|l| l.chars().count() as i32).sum();

    (raw_chars - actual_chars).to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    let new_string_chars: i32 = input.lines().map(|l| re_encode_and_count(l)).sum();
    let raw_chars: i32 = input.lines().map(|l| l.chars().count() as i32).sum();

    (new_string_chars - raw_chars).to_string()
}

#[test]
fn test_run() {
    assert_eq!(count_actual_chars(r#""""#), 0);
    assert_eq!(count_actual_chars(r#""abc""#), 3);
    assert_eq!(count_actual_chars(r#""aaa\"aaa""#), 7);
    assert_eq!(count_actual_chars(r#""\x27""#), 1);
}

#[test]
fn test_run_pt2() {
    assert_eq!(re_encode_and_count(r#""""#), 6);
    assert_eq!(re_encode_and_count(r#""abc""#), 9);
    assert_eq!(re_encode_and_count(r#""aaa\"aaa""#), 16);
    assert_eq!(re_encode_and_count(r#""\x27""#), 11);
}