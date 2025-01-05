fn main() {
    dbg!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]));
}

struct Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut lcis = 1;
        let mut l = 0_usize;

        for idx in 1..nums.len() {
            if nums[idx] <= nums[idx - 1] {
                lcis = lcis.max(idx - l);
                l = idx;
            }
        }
        lcis.max(nums.len() - l) as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_length_of_lcis() {
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    }
}
