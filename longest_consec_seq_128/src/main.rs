fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    // reworked python solution with help from NaveenDA:
    // https://leetcode.com/problems/longest-consecutive-sequence/solutions/5047720/rust-solution
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: std::collections::HashSet<&i32> = nums.iter().collect();
        let mut max_len = 0;

        for &i in set.iter() {
            if !set.contains(&(i - 1)) {
                let mut j = 0;
                while set.contains(&(i + j)) {
                    j += 1;
                }
                max_len = std::cmp::max(max_len, j);
            }
        }
        max_len
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
