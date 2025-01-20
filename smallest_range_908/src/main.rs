fn main() {
    dbg!(Solution::smallest_range_i(vec![1, 3, 6], 3));
}

struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        0.max((nums.iter().max().unwrap() - k) - (nums.iter().min().unwrap() + k))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_smallest_range_i() {
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
