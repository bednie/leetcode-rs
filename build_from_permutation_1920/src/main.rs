fn main() {
    dbg!(Solution::build_array(vec![5, 0, 1, 2, 3, 4]));
}

struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|n| nums[*n as usize]).collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_build_array() {
        assert_eq!(
            Solution::build_array(vec![5, 0, 1, 2, 3, 4]),
            vec![4, 5, 0, 1, 2, 3]
        );
    }
}
