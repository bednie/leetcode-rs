fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        queue.push(root.clone());

        while !queue.is_empty() {
            let length = queue.len();
            let mut level: Vec<i32> = vec![];

            (1..=length).for_each(|_i: usize| {
                if let Some(node) = queue.remove(0) {
                    let current_node = node.borrow();
                    level.push(current_node.val);
                    queue.push(current_node.left.to_owned());
                    queue.push(current_node.right.to_owned());
                }
            });
            if !level.is_empty() {
                result.push(level);
            }
        }
        result
    }
}
