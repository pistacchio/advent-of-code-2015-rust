use std::fs::read_to_string;
use itertools::{Itertools, min};

const INPUT_FILE: &str = "data/day_24.txt";

fn compute_qe(presents: &[u128], num_groups: u128) -> u128 {
    let presents_sum: u128 = presents.iter().sum::<u128>();
    let group_size: u128 = presents_sum / num_groups;

    for i in 0..group_size {
        let qes: Vec<u128> = presents.iter()
            .cloned()
            .combinations(i as usize)
            .filter(|c| c.iter().sum::<u128>() == group_size)
            .map(|c| c.iter().product())
            .collect();

        if !qes.is_empty() {
            return min(qes).unwrap()
        }
    }

    0
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let presents: Vec<u128> = input.lines().map(|l| l.parse::<u128>().unwrap()).collect();

    compute_qe(&presents, 3).to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let presents: Vec<u128> = input.lines().map(|l| l.parse::<u128>().unwrap()).collect();

    compute_qe(&presents, 4).to_string()
}

#[test]
fn test() {
    assert_eq!(compute_qe(&[1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 3), 99);
}
