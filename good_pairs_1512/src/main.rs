fn main() {
    dbg!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
}

struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut good_pairs = 0;

        for idx_1 in 0..nums.len() - 1 {
            for idx_2 in idx_1 + 1..nums.len() {
                if nums[idx_1] == nums[idx_2] {
                    good_pairs += 1;
                }
            }
        }
        good_pairs
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }
}
