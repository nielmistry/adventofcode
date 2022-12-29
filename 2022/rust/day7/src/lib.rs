use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// Reference: https://developerlife.com/2022/02/24/rust-non-binary-tree/#naive-approach-using-weak-and-strong-references
type NodeDataRef<T> = Rc<NodeData<T>>;
type WeakNodeDataRef<T> = Weak<NodeData<T>>;

type Parent<T> = RefCell<WeakNodeDataRef<T>>;
type Children<T> = RefCell<Vec<Child<T>>>;
type Child<T> = NodeDataRef<T>;

struct NodeData<T> {
    value: T,
    parent: Parent<T>,
    children: Children<T>,
}

struct Node<T> {
    r: NodeDataRef<T>,
}

impl<T> Deref for Node<T> {
    type Target = NodeData<T>;
    fn deref(&self) -> &Self::Target {
        &self.r
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        let new_node = NodeData {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        };
        let rc_ref = Rc::new(new_node);
        Node { r: rc_ref }
    }

    pub fn get_node_ref_copy(self: &Self) -> NodeDataRef<T> {
        Rc::clone(&self.r)
    }

    pub fn create_and_add_child(self: &Self, child_value: T) -> NodeDataRef<T> {
        let child_node = Node::new(child_value);

        {
            let mut children = self.children.borrow_mut();
            children.push(child_node.get_node_ref_copy());
        }
        {
            let mut childs_parent = child_node.parent.borrow_mut();
            *childs_parent = Rc::downgrade(&self.get_node_ref_copy());
        }

        child_node.get_node_ref_copy()
    }

    pub fn get_parent(self: &Self) -> Option<NodeDataRef<T>> {
        let parent = self.parent.borrow();
        let node_data = parent.upgrade();
        if let Some(data) = node_data {
            Some(data)
        } else {
            None
        }
    }
}

struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    pub fn new(value: T) -> Tree<T> {
        Tree {
            root: Node::new(value),
        }
    }
}

pub fn part1(input: &str) -> usize {
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{borrow::Borrow, fs};

    #[test]
    fn tree_test() {
        let tree: Tree<u32> = Tree::new(0);
        tree.root.create_and_add_child(10);
        tree.root.create_and_add_child(20);

        let children = tree.root.children.borrow();
        assert_eq!(children.len(), 2);
        let child1 = children.get(0).unwrap();
        assert_eq!(child1.value, 10);

        let parent = *child1.parent.borrow();
    }

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
