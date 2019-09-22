use std::collections::HashMap;
use permutohedron::LexicalPermutation;
use std::iter::{FromIterator, repeat};
use std::fs::read_to_string;

const INPUT_FILE: &str = "data/day_13.txt";

fn parse_hosts(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut hosts: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for line in input.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        let host = words[0];
        let mut happiness = words[3].parse::<i32>().unwrap();
        let friend = words[10].trim_end_matches('.');
        if words[2] == "lose" {
            happiness *= -1;
        }

        hosts.entry(host.to_owned()).or_insert_with(HashMap::new).insert(friend.to_owned(), happiness);
    }

    hosts
}

fn arrange_table(hosts: HashMap<String, HashMap<String, i32>>) -> i32 {

    let mut data: Vec<String> = hosts.keys().cloned().collect();
    let mut permutations: Vec<Vec<String>> = Vec::new();

    loop {
        permutations.push(data.clone());
        if !data.next_permutation() {
            break;
        }
    }

    // make the permutations circular
    permutations = permutations.iter_mut().map(|p| {
        p.push(p[0].clone());
        p.to_owned()
    }).collect();

    let happiness: Vec<i32> = permutations.iter().map(|table| -> i32 {
        table.iter().enumerate().map(|(idx, host)| {
            let mut host_happiness = 0;
            if idx != 0 {
                host_happiness += hosts[host][&table[idx - 1]];
            }
            if idx != table.len() - 1 {
                host_happiness += hosts[host][&table[idx + 1]];
            }

            host_happiness
        }).sum()
    }).collect();

    *happiness.iter().max().unwrap()
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    arrange_table(parse_hosts(input)).to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    let mut hosts = parse_hosts(input);
    hosts.insert("Me".to_string(), HashMap::from_iter(hosts.keys().cloned().zip(repeat(0)).collect::<Vec<(String, i32)>>()));

    hosts = hosts.iter_mut().map(|(key, neighbours)| {
        neighbours.insert("Me".to_string(), 0);
        (key.clone(), neighbours.clone())
    }).collect();

    arrange_table(hosts).to_string()
}

#[test]
fn test_run() {
    let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

    assert_eq!(arrange_table(parse_hosts(input)), 330);
}
