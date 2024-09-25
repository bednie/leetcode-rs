fn main() {
    dbg!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
}

struct Solution;

// impl Solution {
//     pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//         let mut n = nums.clone();
//         n.sort_unstable();
//         n.dedup();
//         n.len() != nums.len()
//     }
// }

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut s =  std::collections::HashSet::new();
        nums.into_iter().any(|n| {
            !s.insert(n)
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }
}
