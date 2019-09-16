use serde_json::value::Value;
use std::borrow::Borrow;
use std::fs::read_to_string;

const INPUT_FILE: &str = "data/day_12.txt";

fn get_value(value: Value) -> i64 {
    match value {
        Value::Array(vals) => vals.iter().cloned().map(get_value).sum(),
        Value::Object(vals) => vals.values().cloned().map(get_value).sum(),
        Value::Number(val) => val.as_i64().unwrap(),
        _ => 0
    }
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    let json: Value = serde_json::from_str(input).unwrap();

    get_value(json).to_string()
}

pub fn run_pt2() -> String {
    "".to_string()
}

#[test]
fn test_run() {
    let json: Value = serde_json::from_str(r#"[1,2,3]"#).unwrap();
    assert_eq!(get_value(json), 6);

    let json: Value = serde_json::from_str(r#"{"a":2,"b":4}"#).unwrap();
    assert_eq!(get_value(json), 6);

    let json: Value = serde_json::from_str(r#"[[[3]]]"#).unwrap();
    assert_eq!(get_value(json), 3);

    let json: Value = serde_json::from_str(r#"{"a":{"b":4},"c":-1}"#).unwrap();
    assert_eq!(get_value(json), 3);

    let json: Value = serde_json::from_str(r#"{"a":[-1,1]}"#).unwrap();
    assert_eq!(get_value(json), 0);

    let json: Value = serde_json::from_str(r#"[]"#).unwrap();
    assert_eq!(get_value(json), 0);

    let json: Value = serde_json::from_str(r#"{}"#).unwrap();
    assert_eq!(get_value(json), 0);

}

