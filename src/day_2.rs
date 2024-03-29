use std::fs::read_to_string;
use itertools::Itertools;

const INPUT_FILE: &str = "data/day_2.txt";

struct Gift {
    l: u32,
    w: u32,
    h: u32,

    sides: [u32; 3],
}

impl Gift {
    fn smallest_side(&self) -> u32 {
        *self.sides.iter().min().unwrap()
    }
}

impl From<&str> for Gift {
    fn from(s: &str) -> Self {
        let sizes = s.split('x').collect::<Vec<&str>>();
        let l = sizes[0].parse().unwrap();
        let w = sizes[1].parse().unwrap();
        let h = sizes[2].parse().unwrap();

        Self {
            l,
            w,
            h,
            sides: [
                l * w,
                w * h,
                h * l
            ],
        }
    }
}

fn compute_wapping_paper(input: &str) -> u32 {
    let gifts: Vec<Gift> = input.lines().map(|g| g.into()).collect();

    gifts.iter().map(|gift| {
        let sides_sum: u32 = gift.sides.iter().map(|s| s * 2).sum();
        sides_sum + gift.smallest_side()
    }).sum()
}

fn compute_ribbon(input: &str) -> u32 {
    let gifts: Vec<Gift> = input.lines().map(|g| g.into()).collect();

    gifts.iter().map(|gift| {
        let dimensions = [gift.l, gift.w, gift.h];
        let smallest_dimensions_sum: u32 = dimensions.iter()
            .sorted()
            .take(2)
            .map(|d| d * 2)
            .sum();
        let all_dimensions_sum: u32 = dimensions.iter().product();

        smallest_dimensions_sum + all_dimensions_sum
    }).sum()
}

pub fn run() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    compute_wapping_paper(input).to_string()
}

pub fn run_pt2() -> String {
    let input = read_to_string(INPUT_FILE).unwrap();
    let input = input.trim();

    compute_ribbon(input).to_string()
}

#[test]
fn test_run() {
    assert_eq!(compute_wapping_paper("2x3x4"), 58);
    assert_eq!(compute_wapping_paper("1x1x10"), 43);
}

#[test]
fn test_run_pt2() {
    assert_eq!(compute_ribbon("3x2x4"), 34);
    assert_eq!(compute_ribbon("1x1x10"), 14);
}