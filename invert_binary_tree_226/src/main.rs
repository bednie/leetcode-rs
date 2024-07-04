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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let node = root.clone();
        let mut node = node.borrow_mut();
        let (l, r) = (node.left.take(), node.right.take());
        (node.left, node.right) = (Self::invert_tree(r), Self::invert_tree(l));
        Some(root)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_invert_tree() {
        let t = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        };

        let u = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        };

        let v = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        };

        let w = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        };

        let u_inverted = Solution::invert_tree(Some(Rc::new(RefCell::new(u)))).unwrap();

        assert_eq!(u_inverted, Rc::new(RefCell::new(t)));
        assert_ne!(Rc::new(RefCell::new(v)), Rc::new(RefCell::new(w)));
    }
}
