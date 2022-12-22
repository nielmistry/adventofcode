use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    size: Option<u32>,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            size: None,
            parent: None,
            children: vec![],
        };
    }
}

pub fn part1(input: &str) -> usize {
    let a = Node::new();
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&input), 95437);
    }

    // #[test]
    // fn part2_test() {
    //     let input = fs::read_to_string("test.txt").unwrap();
    //     let ans = part2(&input);
    //     assert_eq!("MCD", ans);
    // }
}
