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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }

        let inorder = inorder.clone();
        let mut postorder = postorder.clone();

        let mut root = TreeNode::new(postorder.pop()?);

        // get index
        let index = inorder.iter().position(|&x| x == root.val).unwrap();
        
        // split vecs
        let (li, ri) = inorder.split_at(index as usize);
        let (lp, rp) = postorder.split_at(index as usize);

        // left
        root.left = Self::build_tree(li.to_vec(), lp.to_vec());

        // right
        root.right = Self::build_tree(ri[1..].to_vec(), rp.to_vec());

        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_build_tree() {
        let t = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        };

        assert_eq!(
            Solution::build_tree(vec![1, 2, 5], vec![1, 5, 2]),
            Some(Rc::new(RefCell::new(t)))
        );
    }
}
