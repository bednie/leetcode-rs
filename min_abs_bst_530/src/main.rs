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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn in_order(node: &Option<Rc<RefCell<TreeNode>>>, mut v: &mut Vec<i32>) {
            if let Some(n) = node {
                in_order(&n.borrow().left, &mut v);
                v.push(n.borrow().val);
                in_order(&n.borrow().right, &mut v);
            } else { 
                return
            }
        }

        let mut v: Vec<i32> = vec![];
        in_order(&root, &mut v);

        let (mut min, mut left, mut right) = (i32::MAX, 0, v.len() - 1);
        while left < right {
            min = std::cmp::min(min, v[left + 1] - v[left]);
            left += 1;
        }
        min
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_get_minimum_difference() {
        let t = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        };

        assert_eq!(Solution::get_minimum_difference(Some(Rc::new(RefCell::new(t)))), 1);
    }
}
