use std::fs::read_to_string;
use permutator::{Combination};
use std::collections::HashSet;

const INPUT_FILE: &str = "data/day_17.txt";

fn put_eggnod_in_containers(liters: i32, container_sizes: &[i32]) -> HashSet<Vec<(usize, i32)>> {
    let container_sizes: Vec<_> = container_sizes.iter().copied().enumerate().collect();

    let mut combinations = HashSet::new();

    for n in 1..=container_sizes.len() {
        for combination in container_sizes.combination(n) {
            let sum: i32 = combination.iter().map(|c| c.1).sum();
            if sum == liters {
                combinations.insert(combination);
            }
        }
    }

    combinations.into_iter().map(|v| v.into_iter().copied().collect()).collect()
}

fn put_eggnod_in_containers_minimum(liters: i32, container_sizes: &[i32]) -> usize {
    let combinations = put_eggnod_in_containers(liters, container_sizes);

    let min_containers = combinations.iter().map(|c| c.len()).min().unwrap();
    combinations.iter().filter(|c| c.len() == min_containers).count()
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();

    let container_sizes: Vec<_> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    put_eggnod_in_containers(150, &container_sizes).len().to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();

    let container_sizes: Vec<_> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    put_eggnod_in_containers_minimum(150, &container_sizes).to_string()
}

#[test]
fn test_run() {
    assert_eq!(put_eggnod_in_containers(25, &[20, 15, 10, 5, 5]).len(), 4);
}

#[test]
fn test_run_pt2() {
    assert_eq!(put_eggnod_in_containers_minimum(25, &[20, 15, 10, 5, 5]), 3);
}
