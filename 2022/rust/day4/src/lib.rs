#![feature(iter_array_chunks)]
use std::cmp::Ordering;
use std::ops::Range;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let range_bounds = x.split(",").collect::<Vec<_>>();
            let first_elf_range = range_bounds
                .first()
                .unwrap()
                .split("-")
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let second_elf_range = range_bounds
                .last()
                .unwrap()
                .split("-")
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

            match first_elf_range[0].cmp(&second_elf_range[0]) {
                Ordering::Less => {
                    // The first elf range starts outside the second elf range, so check if it also ends outside
                    match first_elf_range[1].cmp(&second_elf_range[1]) {
                        Ordering::Greater | Ordering::Equal => 1,
                        _ => 0,
                    }
                }
                Ordering::Greater => {
                    // first elf range starts inside the second elf range, so check if it also ends inside
                    match first_elf_range[1].cmp(&second_elf_range[1]) {
                        Ordering::Less | Ordering::Equal => 1,
                        _ => 0,
                    }
                }
                Ordering::Equal => 1, // if the first element is the same, at least one range is inside the other!
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let range_bounds = x.split(",").collect::<Vec<_>>();
            let first_elf_range = range_bounds
                .first()
                .unwrap()
                .split("-")
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let second_elf_range = range_bounds
                .last()
                .unwrap()
                .split("-")
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

            match first_elf_range[0].cmp(&second_elf_range[0]) {
                Ordering::Less => {
                    // The first elf range starts outside the second elf range, so check if the end of the first range encompasses the beginning of the second range
                    match first_elf_range[1].cmp(&second_elf_range[0]) {
                        Ordering::Greater | Ordering::Equal => 1,
                        _ => 0,
                    }
                }
                Ordering::Greater => {
                    // first elf range starts inside the second elf range, so check if the end of the second range is inside
                    match first_elf_range[0].cmp(&second_elf_range[1]) {
                        Ordering::Less | Ordering::Equal => 1,
                        _ => 0,
                    }
                }
                Ordering::Equal => 1, // if the first element is the same, at least one range is inside the other!
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let ans = part1(&input.to_string());
        assert_eq!(2, ans);
    }

    #[test]
    fn part2_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let ans = part2(&input.to_string());
        assert_eq!(4, ans);
    }
    #[test]
    fn part2_test_custom() {
        let input = "1-3,3-5
10-20,21-50
2-6,4-5
4-5,2-6";
        let ans = part2(&input.to_string());
        assert_eq!(3, ans);
    }
}
