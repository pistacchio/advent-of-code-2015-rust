use std::collections::HashMap;

use itertools::Itertools;

fn find_house(n: u64) -> u64 {
    let mut houses: HashMap<u64, u64> = HashMap::new();

    for i in 1..=(n / 10) {
        for j in (i..=(n / 10)).step_by(i as usize) {
            *houses.entry(j).or_insert(0) += i * 10;
        }
    }

    houses.into_iter()
        .filter_map(|(k, v)| if v >= n { Some(k)} else {None})
        .sorted()
        .collect::<Vec<_>>()[0]
        .to_owned()
}

fn find_house_lazy_elves(n: u64) -> u64 {
    let mut houses: HashMap<u64, u64> = HashMap::new();

    for i in 1..=(n / 10) {
        for j in (i..=(n / 10)).step_by(i as usize).take(50) {
            *houses.entry(j).or_insert(0) += i * 11;
        }
    }

    houses.into_iter()
        .filter_map(|(k, v)| if v >= n { Some(k)} else {None})
        .sorted()
        .collect::<Vec<_>>()[0]
        .to_owned()
}

pub fn run() -> String {
    find_house(34_000_000).to_string()
}

pub fn run_pt2() -> String {
    find_house_lazy_elves(34_000_000).to_string()
}

#[test]
fn test_get_house_presents() {
    assert_eq!(find_house(1), 10);
    assert_eq!(find_house(2), 30);
    assert_eq!(find_house(9), 130);
}
