use std::collections::BTreeSet;

use priority_queue::PriorityQueue;

trait PushWithAppend {
    fn push_with_append(&mut self, k: &char); // TODO: How to make this more generic?
}

impl PushWithAppend for PriorityQueue<char, u16> {
    fn push_with_append(&mut self, k: &char) {
        // first check if the key is inside..
        let found_key = self.get(k);
        match found_key {
            Some(_v) => {
                self.change_priority_by(k, |old_prio| *old_prio = *old_prio + 1);
            }
            None => {
                self.push(*k, 1);
            }
        };
    }
}

fn do_work(input: &str, num_chars: usize) -> usize {
    let mut q: PriorityQueue<char, u16> = PriorityQueue::new();
    let char_vec = input.chars().enumerate().collect::<Vec<(usize, char)>>();
    for (i, c) in char_vec {
        q.push_with_append(&c);
        if i >= num_chars {
            // nth is gross, probably a better way of doing this.. an obvious alternate is have is a copy of the vec? Borrow checker screws up attempts to just reuse the char_vec vec
            // Thinking perhaps having
            let c_to_reduce = &input.chars().nth(i - num_chars).unwrap();

            // start removing the last one
            let to_reduce = q.get_mut(c_to_reduce).unwrap();
            if *to_reduce.1 == 1 {
                // just remove this from the queue
                q.remove(c_to_reduce);
            } else {
                q.change_priority_by(c_to_reduce, |prio| {
                    *prio -= 1;
                });
            }

            // every time we pop after reaching min, we should check the top of the prio queue. If its 1, then we know everything is 1.
            match q.peek() {
                Some(v) => {
                    if *v.1 == 1 {
                        return i + 1;
                    }
                }
                _ => panic!("There should always be a top!"),
            };
        }
    }

    0
}

fn better_way(input: &str, window_size: usize) -> usize {
    let char_vec = input.chars().collect::<Vec<char>>();

    let output = char_vec
        .windows(window_size)
        .enumerate()
        .find(|(i, x)| {
            let set = x.iter().collect::<BTreeSet<&char>>();
            x.len() == set.len()
        })
        .unwrap();
    output.0 + window_size
}

pub fn part1(input: &str) -> usize {
    // do_work(input, 4)
    better_way(input, 4)
}

pub fn part2(input: &str) -> usize {
    do_work(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let input1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let ans = part1(&input1);
        assert_eq!(7, ans);
        let input2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let ans = part1(&input2);
        assert_eq!(5, ans);
        let input3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let ans = part1(&input3);
        assert_eq!(6, ans);
    }

    // #[test]
    // fn part2_test() {
    //     let input = fs::read_to_string("test.txt").unwrap();
    //     let ans = part2(&input);
    //     assert_eq!("MCD", ans);
    // }
}
