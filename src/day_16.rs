use std::fs::read_to_string;
use std::collections::HashMap;
use regex::{Regex};
use std::iter::FromIterator;


const INPUT_FILE: &str = "data/day_16.txt";
const MFCSAM_MESSAGE: &str = r#"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"#;

lazy_static! {
    static ref AUNT_REGEX: Regex = Regex::new(r"^Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)$").unwrap();
}

type Aunt = HashMap<String, i32>;
type Aunts = Vec<Aunt>;
type MFCSAMMessage = HashMap<String, i32>;

fn register_aunts(input: &[&str]) -> Aunts {
    let mut aunts = vec![];

    for aunt_raw in input {
        if let Some(aunt_props) = AUNT_REGEX.captures(aunt_raw) {
            let aunt_props = aunt_props.iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().to_string())
                .collect::<Vec<String>>();

            aunts.push(HashMap::from_iter(vec![
                ("number".to_owned(), aunt_props[0].parse::<i32>().unwrap()),
                (aunt_props[1].to_owned(), aunt_props[2].parse::<i32>().unwrap()),
                (aunt_props[3].to_owned(), aunt_props[4].parse::<i32>().unwrap()),
                (aunt_props[5].to_owned(), aunt_props[6].parse::<i32>().unwrap()),
            ]));
        }
    }

    aunts
}

fn register_mfcsam_message(input: &[&str]) -> MFCSAMMessage {
    let mut message = HashMap::new();

    for item in input {
        let words: Vec<_> = item.split_whitespace().collect();
        message.insert(words[0].trim_end_matches(':').to_owned(), words[1].parse::<i32>().unwrap());
    }

    message
}

fn match_aunt(aunts: &Aunts, mfcsam_message: &MFCSAMMessage) -> Option<Aunt> {
    aunts.iter()
        .find(|aunt| {
            for (prop_name, prop_val) in aunt.iter() {
                if prop_name == "number" {
                    continue;
                }

                if let Some(message_val) = mfcsam_message.get(prop_name) {
                    if message_val != prop_val {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            true
        }).cloned()
}

fn match_aunt_exact(aunts: &Aunts, mfcsam_message: &MFCSAMMessage) -> Option<Aunt> {
    aunts.iter()
        .find(|aunt| {
            for (prop_name, prop_val) in aunt.iter() {
                if prop_name == "number" {
                    continue;
                }

                if let Some(message_val) = mfcsam_message.get(prop_name) {
                    match prop_name.as_ref() {
                        "cats" | "trees" => {
                            if prop_val <= message_val {
                                return false;
                            }
                        }
                        "pomeranians" | "goldfish" => {
                            if prop_val >= message_val  {
                                return false;
                            }
                        }
                        _ => {
                            if message_val != prop_val {
                                return false;
                            }
                        }
                    }
                } else {
                    return false;
                }
            }

            true
        }).cloned()

}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();

    let aunts = register_aunts(&input.lines().collect::<Vec<_>>());

    let mfcsam_message = register_mfcsam_message(&MFCSAM_MESSAGE.lines().collect::<Vec<_>>());
    match_aunt(&aunts, &mfcsam_message).unwrap()["number"].to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();

    let aunts = register_aunts(&input.lines().collect::<Vec<_>>());

    let mfcsam_message = register_mfcsam_message(&MFCSAM_MESSAGE.lines().collect::<Vec<_>>());
    match_aunt_exact(&aunts, &mfcsam_message).unwrap()["number"].to_string()
}
