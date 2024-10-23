fn main() {
    dbg!(Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]));
}

struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums3 = 0;

        for n1 in nums1 {
            for n2 in &nums2 {
                nums3 ^= n1 ^ n2;
            }
        }
        nums3
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_xor_all_nums() {
        assert_eq!(Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
    }
}
