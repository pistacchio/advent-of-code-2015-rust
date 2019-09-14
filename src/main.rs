#[macro_use] extern crate lazy_static;


mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_4_par;
mod day_5;
mod day_6;
mod day_7;

fn main() {
    println!("Day 1: {}", day_1::run());
    println!("Day 1, pt2: {}", day_1::run_pt2());
    println!("Day 2: {}", day_2::run());
    println!("Day 2, pt2: {}", day_2::run_pt2());
    println!("Day 3: {}", day_3::run());
    println!("Day 3, pt2: {}", day_3::run_pt2());
    println!("Day 4: {}", day_4::run());
    println!("Day 4, pt2: {}", day_4::run_pt2());
    println!("Day 4, par: {}", day_4_par::run());
    println!("Day 4, par, pt2: {}", day_4_par::run_pt2());
    println!("Day 5: {}", day_5::run());
    println!("Day 5, pt2: {}", day_5::run_pt2());
    println!("Day 6: {}", day_6::run());
    println!("Day 6, pt2: {}", day_6::run_pt2());
    println!("Day 7: {}", day_7::run());
    println!("Day 7, pt2: {}", day_7::run_pt2());
}
