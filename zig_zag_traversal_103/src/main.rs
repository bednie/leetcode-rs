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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut q = std::collections::VecDeque::new();
        if let Some(r) = root {
            q.push_back(r);
        }

        while !q.is_empty() {
            let mut temp: std::collections::VecDeque<Rc<RefCell<TreeNode>>> =
                std::collections::VecDeque::new();
            let mut vals: Vec<i32> = vec![];
            while !q.is_empty() {
                let node = q.pop_front();
                if let Some(ref n) = node {
                    let val = n.borrow().val.clone();
                    vals.push(val);
                }

                if let Some(ref l) = node {
                    if let Some(left) = l.borrow().left.clone() {
                        temp.push_back(left);
                    }
                }

                if let Some(ref r) = node {
                    if let Some(right) = r.borrow().right.clone() {
                        temp.push_back(right);
                    }
                }
            }

            if !vals.is_empty() {
                result.push(vals);
            }

            if !temp.is_empty() {
                q = temp;
            }
        }

        let mut ans = vec![];
        for (i, item) in result.into_iter().enumerate() {
            if i % 2 == 1 {
                ans.push(item.into_iter().rev().collect::<Vec<i32>>());
            } else {
                ans.push(item);
            }
        }
        ans
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_zigzag_level_order() {
        //t  u  v u.left u.right v.left, v.right
        // [3,9,20,null,  null,    15,      7]
        let u = TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        };

        let t = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(u))),
        };

        let uu = TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        };

        let tt = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(uu))),
        };

        assert_eq!(
            Solution::zigzag_level_order(Some(Rc::new(RefCell::new(t)))),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );

        assert_ne!(
            Solution::zigzag_level_order(Some(Rc::new(RefCell::new(tt)))),
            vec![vec![3], vec![9, 20], vec![7, 15]]
        );
    }
}
