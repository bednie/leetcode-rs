fn main() {
    dbg!(Solution::sum_indices_with_k_set_bits(
        vec![5, 10, 1, 5, 2],
        1
    ));
}

struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().enumerate().fold(0, |acc, (idx, num)| {
            if idx.count_ones() == k as u32 {
                acc + num
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sum_indices_with_k_set_bits() {
        assert_eq!(
            Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1),
            13
        );
    }
}
