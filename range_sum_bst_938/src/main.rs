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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let root = root.borrow();

        match root.val {
            n if n >= low && n <= high => {
                root.val
                    + Self::range_sum_bst(root.left.clone(), low, high)
                    + Self::range_sum_bst(root.right.clone(), low, high)
            }

            n if n < low => Self::range_sum_bst(root.right.clone(), low, high),

            n if n > high => Self::range_sum_bst(root.left.clone(), low, high),

            _ => panic!("something went wrong!"),
        }
    }
}
