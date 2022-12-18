use day5::part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Output: {}", part1(&file));
}
