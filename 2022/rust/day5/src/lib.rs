use itertools::Itertools;
use nom::{
    self,
    bytes::complete::{is_not, take_till, take_while, take_while1},
    character::{is_alphabetic, is_digit},
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

struct Operation {
    num_to_move: usize,
    src: usize,
    dst: usize,
}

fn skip_text(input: &str) -> IResult<&str, &str> {
    take_while(|x: char| !x.is_digit(10))(input)
}

fn get_num(input: &str) -> IResult<&str, &str> {
    take_while1(|x: char| x.is_digit(10))(input)
}

fn get_next_num(input: &str) -> IResult<&str, usize> {
    let (input, _) = skip_text(input)?;
    let (input, num) = get_num(input)?;
    Ok((input, num.parse::<usize>().unwrap()))
}

fn parse_command(input: &str) -> IResult<&str, Operation> {
    let (input, (num_to_move, src, dst)) =
        tuple((get_next_num, get_next_num, get_next_num))(input)?;

    Ok((
        input,
        Operation {
            num_to_move,
            src,
            dst,
        },
    ))
}

fn print_stack_states(input: &HashMap<usize, Vec<char>>) {
    for i in 0..input.len() {
        println!("Stack {i}:");
        for c in input.get(&i).unwrap() {
            println!("{c}");
        }
    }
}

pub fn part1(input: &str) -> &str {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    let (stack_lines, operations) = input.split("\n\n").collect_tuple().unwrap(); // Split at the double new line

    // 1. parse the current state of the stacks, store them in the hashmap
    stack_lines.lines().for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .inspect(|x| println!("This is x: {x}"))
            .enumerate()
            .for_each(|(stack, c)| {
                if c.is_alphabetic() {
                    println!("{c} is alphabetic: {stack}");
                    // we must put it in
                    if !stacks.contains_key(&stack) {
                        stacks.insert(stack, Vec::new());
                    }

                    stacks.get_mut(&stack).unwrap().push(c);
                }
            })
    });

    // 2. parse operations line by line and

    println!("{}", operations);
    for operation in operations.lines() {
        let (_, op) = parse_command(operation).unwrap();
        let s_stack = stacks.get_mut(&op.src).unwrap();
        let mut tmp_stack: Vec<char> = Vec::with_capacity(op.num_to_move);
        for _ in 0..op.num_to_move {
            tmp_stack.push(s_stack.pop().unwrap());
        }
        tmp_stack.reverse();
        let d_stack = stacks.get_mut(&op.dst).unwrap();
        for _ in 0..op.num_to_move {
            d_stack.push(tmp_stack.pop().unwrap());
        }

        print_stack_states(&stacks);
    }

    "TET"
}

pub fn part2(input: &str) -> &str {
    "TEST"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        let ans = part1(&input);
        assert_eq!("CMZ", ans);
    }
}
