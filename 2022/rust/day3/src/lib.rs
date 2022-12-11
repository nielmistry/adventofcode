#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::io::{stdout, Write};

fn get_char_priority(character: char) -> u32 {
    let char_code = character as u32;
    let output = match character {
        'a'..='z' => 1 + (char_code - 'a' as u32),
        'A'..='Z' => 27 + (char_code - 'A' as u32),
        _ => panic!("Not a good character!"),
    };
    output
}

pub fn part1(input: &String) -> u32 {
    let v = input
        .split("\n")
        .map(|x| {
            let len = x.len();
            let part1 = &x[0..len / 2];
            let part2 = &x[(len / 2)..];

            let entries: HashSet<char> = part1.chars().collect();
            for c in part2.chars() {
                if entries.contains(&c) {
                    return c;
                }
            }

            panic!("Shouldn't have got here!");
        })
        .map(|character| get_char_priority(character))
        .sum();

    v
}
pub fn part2(input: &String) -> u32 {
    let v = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let citem = a
                .chars()
                .find(|char_in_a| b.contains(*char_in_a) && c.contains(*char_in_a));
            citem
        })
        .map(|character| get_char_priority(character.unwrap()))
        .sum();

    v
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
    #[test]
    fn part1_test2() {
        let input = "abbd";
        let ans = part1(&input.to_string());
        assert_eq!(2, ans);
    }
}
