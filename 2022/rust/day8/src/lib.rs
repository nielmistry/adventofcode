use std::fmt::Debug;

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
#[derive(Clone, Copy)]
enum CardinalDirection {
    Left,
    Right,
    Down,
    Up,
}

type Coordinate = (usize, usize);
fn is_hidden(
    dir: CardinalDirection,
    compare_val: u8,
    mut cur_coord: Coordinate,
    size: Coordinate,
    grid: &Vec<Vec<u8>>,
) -> bool {
    match dir {
        CardinalDirection::Up => cur_coord.1 -= 1,
        CardinalDirection::Down => cur_coord.1 += 1,
        CardinalDirection::Left => cur_coord.0 -= 1,
        CardinalDirection::Right => cur_coord.0 += 1,
    };

    let mut current_value = 0;

    if let Some(first_vec) = grid.get(cur_coord.0) {
        if let Some(value) = first_vec.get(cur_coord.1) {
            current_value = *value;
        }
    } else {
        return false;
    }

    if current_value >= compare_val {
        return true;
    } else {
        return is_hidden(dir, compare_val, cur_coord, size, grid);
    }
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
        // left is always going to be populated because of the way we parse.
        if max_card.left < grid_val {
            visible_list.push((coordx, coordy));
            continue;
        }

        if max_card.up != 0 {
            if max_card.up < grid_val {
                visible_list.push((coordx, coordy));
                continue;
            }
        } else {
            // uninitialized up, we must find it
            if !is_hidden(
                CardinalDirection::Up,
                grid_val,
                (coordx, coordy),
                (width, height),
                &grid,
            ) {
                visible_list.push((coordx, coordy));
                continue;
            }
        }

        if max_card.down != 0 {
            if max_card.down < grid_val {
                visible_list.push((coordx, coordy));
                continue;
            }
        } else {
            // uninitialized down, we must find it
            if !is_hidden(
                CardinalDirection::Down,
                grid_val,
                (coordx, coordy),
                (width, height),
                &grid,
            ) {
                visible_list.push((coordx, coordy));
                continue;
            }
        }

        if max_card.right != 0 {
            if max_card.right < grid_val {
                visible_list.push((coordx, coordy));
                continue;
            }
        } else {
            // uninitialized right, we must find it
            if !is_hidden(
                CardinalDirection::Right,
                grid_val,
                (coordx, coordy),
                (width, height),
                &grid,
            ) {
                visible_list.push((coordx, coordy));
                continue;
            }
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
