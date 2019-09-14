use regex::{Regex, Captures};
use std::fs::read_to_string;
use std::collections::HashMap;

const INPUT_FILE: &str = "data/day_7.txt";

lazy_static! {
    static ref ASSIGN_REGEX: Regex = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
    static ref AND_REGEX: Regex = Regex::new(r"^(\w+) AND (\w+) -> (\w+)$").unwrap();
    static ref OR_REGEX: Regex = Regex::new(r"^(\w+) OR (\w+) -> (\w+)$").unwrap();
    static ref NOT_REGEX: Regex = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();
    static ref LSHIFT_REGEX: Regex = Regex::new(r"^(\w+) LSHIFT (\d+) -> (\w+)$").unwrap();
    static ref RSHIFT_REGEX: Regex = Regex::new(r"^(\w+) RSHIFT (\d+) -> (\w+)$").unwrap();
}

#[derive(Debug)]
enum Operation {
    Assign { val: String, signal: String },
    And { signal_s1: String, signal_s2: String, signal: String },
    Or { signal_s1: String, signal_s2: String, signal: String },
    Not { signal_s: String, signal: String },
    LShift { signal_s: String, val: u16, signal: String },
    RShift { signal_s: String, val: u16, signal: String },
}

impl Operation {
    fn from_str(s: &str) -> Operation {
        let parse_params = |op: Captures| {
            op.iter().skip(1).map(|c| c.unwrap().as_str().to_string()).collect::<Vec<String>>()
        };

        if let Some(op) = ASSIGN_REGEX.captures(s) {
            let params = parse_params(op);
            Operation::Assign { val: params[0].clone(), signal: params[1].clone() }
        } else if let Some(op) = AND_REGEX.captures(s) {
            let params = parse_params(op);
            Operation::And { signal_s1: params[0].clone(), signal_s2: params[1].clone(), signal: params[2].clone() }
        } else if let Some(op) = OR_REGEX.captures(s) {
            let params = parse_params(op);
            Operation::Or { signal_s1: params[0].clone(), signal_s2: params[1].clone(), signal: params[2].clone() }
        } else if let Some(op) = NOT_REGEX.captures(s) {
            let params = parse_params(op);
            Operation::Not { signal_s: params[0].clone(), signal: params[1].clone() }
        } else if let Some(op) = LSHIFT_REGEX.captures(s) {
            let params = parse_params(op);
            Operation::LShift { signal_s: params[0].clone(), val: params[1].parse::<u16>().unwrap(), signal: params[2].clone() }
        } else if let Some(op) = RSHIFT_REGEX.captures(s) {
            let params = parse_params(op);
            Operation::RShift { signal_s: params[0].clone(), val: params[1].parse::<u16>().unwrap(), signal: params[2].clone() }
        } else {
            panic!("Unparsable row{}", s);
        }
    }
}

fn get_value(signal: &str, signals: &HashMap<String, Operation>, memo: &mut HashMap<String, u16>) -> u16 {
    {
        if let Some(val) = memo.get(signal) {
            return *val;
        }
    }

    if let Ok(val) = signal.parse::<u16>() {
        return val;
    }

    let operation = signals.get(signal).unwrap();

    let value: u16;

    match &operation {
        Operation::Assign { val, .. } => value = get_value(val, signals, memo),
        Operation::And { signal_s1, signal_s2, .. } => value = get_value(signal_s1, signals, memo) & get_value(signal_s2, signals, memo),
        Operation::Or { signal_s1, signal_s2, .. } => value = get_value(signal_s1, signals, memo) | get_value(signal_s2, signals, memo),
        Operation::Not { signal_s, .. } => value = !get_value(signal_s, signals, memo),
        Operation::LShift { signal_s, val, .. } => value = get_value(signal_s, signals, memo) << *val,
        Operation::RShift { signal_s, val, .. } => value = get_value(signal_s, signals, memo) >> *val,
    };

    memo.insert(signal.to_string(), value);

    value
}

fn wire(input: &str) -> HashMap<String, u16> {
    let lines = input.lines();
    let operations = lines.map(|l| Operation::from_str(l));
    let mut memo: HashMap<String, u16> = HashMap::new();

    let mut signals: HashMap<String, Operation> = HashMap::new();

    for operation in operations {
        let signal = match &operation {
            Operation::Assign { signal, .. } => signal.clone(),
            Operation::And { signal, .. } => signal.clone(),
            Operation::Or { signal, .. } => signal.clone(),
            Operation::Not { signal, .. } => signal.clone(),
            Operation::LShift { signal, .. } => signal.clone(),
            Operation::RShift { signal, .. } => signal.clone(),
        };

        signals.insert(signal, operation);
    }

    signals.iter()
        .map(|(signal_key, _signal_operations)| (signal_key.clone(), get_value(signal_key, &signals, &mut memo)))
        .collect()
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    wire(input)["a"].to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap().replace("44430", &run());
    let input = input.trim();

    wire(input)["a"].to_string()
}

#[test]
fn test_run() {
    let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    assert_eq!(wire(input), "d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456");
}

#[test]
fn test_run_pt2() {}
