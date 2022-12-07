use std::cmp::Ordering;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
enum RPSState {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPSState {
    type Err = ();
    fn from_str(input: &str) -> Result<RPSState, Self::Err> {
        match input {
            "A" | "X" => Ok(RPSState::Rock),
            "B" | "Y" => Ok(RPSState::Paper),
            "C" | "Z" => Ok(RPSState::Scissors),
            _ => Err(()),
        }
    }
}

fn result(their_move: RPSState, your_move: RPSState) -> Ordering {
    match their_move {
        RPSState::Rock => match your_move {
            RPSState::Rock => return Ordering::Equal,
            RPSState::Paper => return Ordering::Greater,
            RPSState::Scissors => return Ordering::Less,
        },
        RPSState::Paper => match your_move {
            RPSState::Rock => return Ordering::Less,
            RPSState::Paper => return Ordering::Equal,
            RPSState::Scissors => return Ordering::Greater,
        },
        RPSState::Scissors => match your_move {
            RPSState::Rock => return Ordering::Greater,
            RPSState::Paper => return Ordering::Less,
            RPSState::Scissors => return Ordering::Equal,
        },
    }
}

fn get_move(their_move: RPSState, intended_result: Ordering) -> RPSState {
    match their_move {
        RPSState::Rock => match intended_result {
            Ordering::Greater => return RPSState::Paper,
            Ordering::Equal => return RPSState::Rock,
            Ordering::Less => return RPSState::Scissors,
        },
        RPSState::Paper => match intended_result {
            Ordering::Greater => return RPSState::Scissors,
            Ordering::Equal => return RPSState::Paper,
            Ordering::Less => return RPSState::Rock,
        },
        RPSState::Scissors => match intended_result {
            Ordering::Greater => return RPSState::Rock,
            Ordering::Equal => return RPSState::Scissors,
            Ordering::Less => return RPSState::Paper,
        },
    }
}

fn game(your_move: RPSState, their_move: RPSState) -> u32 {
    let mut score;
    match your_move {
        RPSState::Rock => score = 1,
        RPSState::Paper => score = 2,
        RPSState::Scissors => score = 3,
    }
    match result(their_move, your_move) {
        Ordering::Less => {}
        Ordering::Equal => score += 3,
        Ordering::Greater => score += 6,
    }

    score
}

pub fn part1(file: &String) -> u32 {
    let v: Vec<&str> = file.split("\r\n").collect();
    let mut total_score = 0;
    for item in v {
        let moves: Vec<&str> = item.split(" ").collect();
        let their_move = RPSState::from_str(moves[0]).unwrap();
        let our_move = RPSState::from_str(moves[1]).unwrap();

        total_score += game(our_move, their_move);
    }
    total_score
}

pub fn part2(file: &String) -> u32 {
    let v: Vec<&str> = file.split("\r\n").collect();
    let mut total_score = 0;

    for item in v {
        let moves: Vec<&str> = item.split(" ").collect();
        let their_move = RPSState::from_str(moves[0]).unwrap();
        let intended_result = match moves[1] {
            "X" => Ordering::Less,
            "Y" => Ordering::Equal,
            "Z" => Ordering::Greater,
            _ => panic!("Unknown Intended Result!"),
        };

        let our_move = get_move(their_move, intended_result);

        total_score += game(our_move, their_move);
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let testinput = "A Y
B X
C Z";
        let result = part1(&testinput.to_string());
        assert_eq!(result, 15);
    }

    #[test]
    fn all_cases() {
        let testinput = "A X
A Y
A Z
B X
B Y
B Z
C X
C Y
C Z";

        let result = part1(&testinput.to_string());
        assert_eq!(
            result,
            (1 + 3) + (2 + 6) + (3 + 0) + (1 + 0) + (2 + 3) + (3 + 6) + (1 + 6) + (2 + 0) + (3 + 3)
        );
    }

    #[test]
    fn their_rocks() {
        let testinput = "A X
A Y
A Z";

        let result = part1(&testinput.to_string());
        assert_eq!(result, (1 + 3) + (2 + 6) + (3 + 0));
    }

    #[test]
    fn their_papers() {
        let testinput = "B X
B Y
B Z";

        let result = part1(&testinput.to_string());
        assert_eq!(result, (1 + 0) + (2 + 3) + (3 + 6));
    }

    #[test]
    fn their_scissors() {
        let testinput = "C X
C Y
C Z";

        let result = part1(&testinput.to_string());
        assert_eq!(result, (1 + 6) + (2 + 0) + (3 + 3));
    }

    #[test]
    fn test_case2() {
        let testinput = "A Y
B X
C Z";
        let result = part2(&testinput.to_string());
        assert_eq!(result, 12);
    }
}
