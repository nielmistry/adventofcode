#![feature(iter_array_chunks)]

pub fn part1(input: &str) -> u32 {
    let itr = input.lines().map(|x| {
        let ranges: Vec<&str> = x.split(",").collect();
        let first_range: Vec<&str> = ranges[0].split("-").collect();
        let first_num = first_range[0].parse::<u32>().unwrap();
        let second_num: u32 = first_range[1].parse::<u32>().unwrap();
    });
    0
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
}
