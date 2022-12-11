use std::collections::HashSet;
use std::io::{stdout, Write};

pub fn part1(input: &String) -> u32 {
    println!("Hello");
    let rucksacks: u8 = input.split("\n").enumerate().map(|x| '0' as u8).sum();
    println!("{}", rucksacks);

    // for rucksack in rucksacks {
    //     let rucksack_len = rucksack.len();
    //     let part_a = &rucksack[0..rucksack_len / 2];
    //     let part_b = &rucksack[(rucksack_len / 2) + 1..];

    //     println!("{} | {}", part_a, part_b);

    //     let entries: HashSet<char> = part_a.chars().collect();
    //     for c in part_b.chars() {
    //         if entries.contains(&c) {
    //             println!("{} -> {}", c, c as u8);
    //             break;
    //         }
    //     }
    // }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let ans = part1(&input.to_string());
        assert_eq!(157, ans);
    }
}
