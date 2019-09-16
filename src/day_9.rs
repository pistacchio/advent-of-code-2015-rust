use std::fs::read_to_string;
use itertools::Itertools;
use permutohedron::LexicalPermutation;

const INPUT_FILE: &str = "data/day_9.txt";

#[derive(Clone)]
struct Path {
    id: usize,
    from: String,
    to: String,
    distance: i32,
}

impl Path {
    fn from_str(id: usize, input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();

        Self {
            id,
            from: parts[0].to_string(),
            to: parts[2].to_string(),
            distance: parts[4].parse::<i32>().unwrap(),
        }
    }
}

fn all_routes(input: &str) -> Vec<(Vec<Path>, i32)> {
    let paths: Vec<Path> = input.lines()
        .enumerate()
        .map(|(idx, p)| Path::from_str(idx, p)).collect();

    let all_cities: Vec<String> = paths.iter()
        .flat_map(|p| vec![p.from.clone(), p.to.clone()])
        .unique()
        .collect();

    let mut data = all_cities.clone();
    let mut permutations = Vec::new();

    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    // use the previous permutations to compute all the possibile route permutations, eg given
    // [Berlin, London, Dublin] => [(Berlin, London), (London, Dublin)]
    // let route_paths: Vec<Vec<(String, String)>> = permutations
    permutations
        .iter()
        .map(|permutation| {
            permutation.iter()
                .zip(permutation.iter().skip(1))
                .map(|(p1, p2)| (p1.clone(), p2.clone()))
                .map(|(r_from, r_to)| paths.iter().find(|p| {
                    (p.from == r_from && p.to == r_to)
                        || (p.from == r_to && p.to == r_from)
                }).unwrap())
                .cloned()
                .collect()
        })
        .map(|rp: Vec<Path>| (rp.clone(), rp.into_iter().map(|p| p.distance).sum()))
        .sorted_by_key(|rp| rp.1)
        .collect::<Vec<(Vec<Path>, i32)>>()
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    let paths = all_routes(input);
    paths.first().unwrap().1.to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    let paths = all_routes(input);
    paths.last().unwrap().1.to_string()
}

#[test]
fn test_run() {
    let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    assert_eq!(all_routes(input).first().unwrap().1, 605);
}

#[test]
fn test_run_pt2() {}