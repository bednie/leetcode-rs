fn main() {
    dbg!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]));
}

struct Solution;

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            nums[0]
        } else {
            Self::triangular_sum(
                nums.windows(2)
                    .map(|w: &[i32]| w.iter().sum::<i32>() % 10)
                    .collect(),
            )
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_triangular_sum() {
        assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8);
    }
}
