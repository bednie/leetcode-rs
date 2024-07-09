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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = node {
                1 + depth(&n.borrow().left)
            } else {
                0
            }
        }

        if let Some(rc) = root {
            let (left, right) = (depth(&rc.borrow().left), depth(&rc.borrow().right));

            if left == right {
                return (2_i32.pow(left as u32)) + Self::count_nodes(rc.borrow().right.clone());
            } else {
                return (2_i32.pow(right as u32)) + Self::count_nodes(rc.borrow().left.clone());
            }
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_count_nodes() {
        let t = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        };

        assert_eq!(Solution::count_nodes(Some(Rc::new(RefCell::new(t)))), 2);
    }
}
