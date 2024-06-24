fn main() {
    println!("Hello, world!");
}

struct Solution;

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
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(ref pp), Some(ref qq)) => {
                let (pp, qq) = (pp.borrow(), qq.borrow());
                pp.val == qq.val
                    && Solution::is_same_tree(pp.left.clone(), qq.left.clone())
                    && Solution::is_same_tree(pp.right.clone(), qq.right.clone())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        assert!(
            Solution::is_same_tree(
                Option::Some(Rc::new(RefCell::new(TreeNode::new(100)))),
                Option::Some(Rc::new(RefCell::new(TreeNode::new(100))))
            ) && !Solution::is_same_tree(
                Option::None,
                Option::Some(Rc::new(RefCell::new(TreeNode::new(100))))
            )
        );
    }
}
