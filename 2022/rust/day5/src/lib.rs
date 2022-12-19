use itertools::Itertools;
use nom::{
    self,
    bytes::complete::{is_not, take_till, take_while, take_while1},
    character::{is_alphabetic, is_digit},
    sequence::tuple,
    IResult,
};
use std::collections::{HashMap, VecDeque};

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
            src: src - 1,
            dst: dst - 1,
        },
    ))
}

fn print_stack_states(input: &HashMap<usize, VecDeque<char>>) {
    for i in 0..input.len() {
        println!("Stack {i}:");
        for c in input.get(&i).unwrap() {
            println!("{c}");
        }
    }
}

fn get_top_states(input: &HashMap<usize, VecDeque<char>>) -> String {
    let mut output: Vec<char> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        output.push(*input.get(&i).unwrap().front().unwrap());
    }

    String::from_iter(output.iter())
}

fn parse_state_of_stacks(input: &str) -> HashMap<usize, VecDeque<char>> {
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
    // 1. parse the current state of the stacks, store them in the hashmap
    input.lines().for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(stack, c)| {
                if c.is_alphabetic() {
                    // we must put it in
                    if !stacks.contains_key(&stack) {
                        stacks.insert(stack, VecDeque::new());
                    }

                    stacks.get_mut(&stack).unwrap().push_back(c);
                }
            })
    });

    stacks
}

pub fn part1(input: &str) -> String {
    let (stack_lines, operations) = input.split("\n\n").collect_tuple().unwrap(); // Split at the double new line

    let mut stacks = parse_state_of_stacks(&stack_lines);

    for operation in operations.lines() {
        let (_, op) = parse_command(operation).unwrap();
        let s_stack = stacks.get_mut(&op.src).unwrap();
        let mut tmp_stack: VecDeque<char> = VecDeque::with_capacity(op.num_to_move);
        for _ in 0..op.num_to_move {
            tmp_stack.push_back(s_stack.pop_front().unwrap());
        }
        let d_stack = match stacks.get_mut(&op.dst) {
            Some(v) => v,
            None => panic!("No reference found for {}", op.dst),
        };
        for _ in 0..op.num_to_move {
            d_stack.push_front(tmp_stack.pop_front().unwrap());
        }
    }

    get_top_states(&stacks)
}

pub fn part2(input: &str) -> String {
    let (stack_lines, operations) = input.split("\n\n").collect_tuple().unwrap(); // Split at the double new line

    let mut stacks = parse_state_of_stacks(&stack_lines);

    for operation in operations.lines() {
        let (_, op) = parse_command(operation).unwrap();
        let s_stack = stacks.get_mut(&op.src).unwrap();
        let mut tmp_stack: VecDeque<char> = VecDeque::with_capacity(op.num_to_move);
        for _ in 0..op.num_to_move {
            tmp_stack.push_back(s_stack.pop_front().unwrap());
        }
        let d_stack = match stacks.get_mut(&op.dst) {
            Some(v) => v,
            None => panic!("No reference found for {}", op.dst),
        };
        for _ in 0..op.num_to_move {
            d_stack.push_front(tmp_stack.pop_back().unwrap()); // pop the back of the temp stack since the order doesn't change :)
        }
    }

    get_top_states(&stacks)
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

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        let ans = part2(&input);
        assert_eq!("MCD", ans);
    }
}
