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

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> i32 {
            match root {
                None => 0,
                Some(rc) => {
                    let node_ref = rc.borrow();
                    sum *= 10;
                    sum += node_ref.val;
                    match (&node_ref.left, &node_ref.right) {
                        (None, None) => sum,
                        (_, _) => helper(&node_ref.left, sum) + helper(&node_ref.right, sum),
                    }
                }
            }
        }
        helper(&root, 0)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sum_numbers() {
        let t = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        };

        assert_eq!(Solution::sum_numbers(Some(Rc::new(RefCell::new(t)))), 25);
    }
}
