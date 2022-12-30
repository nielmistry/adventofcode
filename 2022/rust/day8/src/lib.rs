use enum_iterator::{all, Sequence};
use std::{fmt::Debug, ops::Neg};

#[derive(Clone)]
struct MaximumCardinal {
    left: u8,
    right: u8,
    down: u8,
    up: u8,
}

impl std::fmt::Debug for MaximumCardinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.left, self.up, self.down, self.right)
    }
}
#[derive(Clone, Copy, Sequence, Debug, PartialEq)]
enum CardinalDirection {
    Left,
    Right,
    Down,
    Up,
}

impl Neg for CardinalDirection {
    type Output = CardinalDirection;
    fn neg(self) -> Self::Output {
        match self {
            CardinalDirection::Down => CardinalDirection::Up,
            CardinalDirection::Up => CardinalDirection::Down,
            CardinalDirection::Left => CardinalDirection::Right,
            CardinalDirection::Right => CardinalDirection::Left,
        }
    }
}

type Coordinate = (usize, usize);

fn is_legal_move(
    dir: CardinalDirection,
    cur_coord: Coordinate,
    grid_size: Coordinate,
) -> Result<Coordinate, bool> {
    let proposed_move = match dir {
        CardinalDirection::Left => (-1, 0),
        CardinalDirection::Right => (1, 0),
        CardinalDirection::Down => (0, 1),
        CardinalDirection::Up => (0, -1),
    };
    let new_coord = (
        cur_coord.0 as i32 + proposed_move.0,
        cur_coord.1 as i32 + proposed_move.1,
    );

    let grid_size = (grid_size.0 as i32, grid_size.1 as i32);

    if new_coord.0 < 0
        || new_coord.0 >= grid_size.0
        || new_coord.1 < 0
        || new_coord.1 >= grid_size.1
    {
        return Err(true);
    } else {
        return Ok((new_coord.0 as usize, new_coord.1 as usize));
    }
}

fn get_value(location: Coordinate, grid: &Vec<Vec<u8>>) -> u8 {
    *grid.get(location.1).unwrap().get(location.0).unwrap()
}

fn update_max_cardinal(
    cur_coord: Coordinate,
    max_cardinal: &mut Vec<Vec<MaximumCardinal>>,
    new_value: MaximumCardinal,
) {
    *max_cardinal
        .get_mut(cur_coord.1)
        .unwrap()
        .get_mut(cur_coord.0)
        .unwrap() = new_value;
}

fn explore_direction(
    dir: CardinalDirection,
    cur_coord: Coordinate,
    target: Coordinate,
    grid_size: Coordinate,
    grid: &Vec<Vec<u8>>,
    max_cardinal: &mut Vec<Vec<MaximumCardinal>>,
    max_value_encountered: u8,
) {
    let mut max_cardinal_local = max_cardinal
        .get(cur_coord.0)
        .unwrap()
        .get(cur_coord.1)
        .unwrap()
        .clone();

    let mut max_value = max_value_encountered;
    let cur_value = get_value(cur_coord, grid);
    if cur_value > max_value {
        max_value = cur_value;
    }

    match dir {
        CardinalDirection::Left => max_cardinal_local.left = max_value,
        CardinalDirection::Right => max_cardinal_local.right = max_value,
        CardinalDirection::Down => max_cardinal_local.down = max_value,
        CardinalDirection::Up => max_cardinal_local.up = max_value,
    };

    println!(
        "Explored direction {:?} at {:?}: {:?}, found max value {:?} | max_cardinal: {:?}",
        dir, cur_coord, cur_value, max_value, max_cardinal_local
    );

    update_max_cardinal(cur_coord, max_cardinal, max_cardinal_local);

    if cur_coord == target {
        return;
    } else if let Ok(legal_move) = is_legal_move(-dir, cur_coord, grid_size) {
        explore_direction(
            dir,
            legal_move,
            target,
            grid_size,
            grid,
            max_cardinal,
            max_value,
        );
    } else {
        return;
    }
}

fn is_hidden(
    mut cur_coord: Coordinate,
    grid_size: Coordinate,
    grid: &Vec<Vec<u8>>,
    max_cardinal: &mut Vec<Vec<MaximumCardinal>>,
) -> bool {
    let dir_list = all::<CardinalDirection>().collect::<Vec<_>>();
    let local_max_cardinal = max_cardinal
        .get(cur_coord.0)
        .unwrap()
        .get(cur_coord.1)
        .unwrap()
        .clone();

    let current_value = get_value(cur_coord, grid);

    for dir in dir_list {
        let max_cardinal_directed = match dir {
            CardinalDirection::Left => &local_max_cardinal.left,
            CardinalDirection::Right => &local_max_cardinal.right,
            CardinalDirection::Down => &local_max_cardinal.down,
            CardinalDirection::Up => &local_max_cardinal.up,
        };

        if *max_cardinal_directed == 0 {
            println!(
                "Exploring {:?} direction at coordinate {:?}: {:?}",
                dir, cur_coord, current_value
            );
            // begin exploration, depending on what direction:
            let exploration_start = match dir {
                CardinalDirection::Left => (0, cur_coord.1),
                CardinalDirection::Right => (grid_size.0 - 1, cur_coord.1),
                CardinalDirection::Up => (cur_coord.0, 0),
                CardinalDirection::Down => (cur_coord.0, grid_size.1 - 1),
            };

            explore_direction(
                dir,
                exploration_start,
                cur_coord,
                grid_size,
                grid,
                max_cardinal,
                0,
            );

            println!("State of local max cardinal: {:?}", local_max_cardinal);
        }

        if *max_cardinal_directed < current_value {
            return false;
        }
    }

    true
}

pub fn part1(input: &str) -> u32 {
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().chars().count();

    let mut grid = vec![vec![0 as u8; width]; height];
    let mut max_cardinal = vec![
        vec![
            MaximumCardinal {
                left: 0,
                right: 0,
                down: 0,
                up: 0
            };
            width
        ];
        height
    ];

    for (i, element) in input.lines().enumerate() {
        let mut max_left: u8 = 0;
        for (j, height) in element.chars().enumerate() {
            let val = height.to_digit(10).unwrap() as u8;
            if val > max_left {
                max_left = val;
            }
            grid[i][j] = val;
            if let Some(v) = max_cardinal[i].get_mut(j + 1) {
                v.left = max_left;
            }
        }
    }

    println!("{:?}", grid);
    println!("{:?}", max_cardinal);

    let mut visible_list: Vec<Coordinate> = Vec::new();
    let mut unknown_list: Vec<Coordinate> = Vec::new();
    let mut invisible_list: Vec<Coordinate> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            if x == 0 || y == 0 || x == width - 1 || height == width - 1 {
                visible_list.push((x, y));
            } else {
                unknown_list.push((x, y))
            }
        }
    }

    println!("Visble {:?}", visible_list);
    println!("Unknown {:?}", unknown_list);

    for (coordx, coordy) in unknown_list {
        // Check each cardinal direction, if all of them are >= then we are invisible. if any of them are < we are visible
        let mut max_card = &max_cardinal[coordx][coordy];
        let grid_val = grid[coordx][coordy];

        if is_hidden((coordx, coordy), (width, height), &grid, &mut max_cardinal) {
            invisible_list.push((coordx, coordy));
        } else {
            visible_list.push((coordx, coordy));
        }
    }

    println!("Visble {:?}", visible_list);

    visible_list.len() as u32
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&input), 21);
    }

    // #[test]
    // fn part2_test() {
    //     let input = fs::read_to_string("test.txt").unwrap();
    //     let ans = part2(&input);
    //     assert_eq!(24933642, ans);
    // }
}
