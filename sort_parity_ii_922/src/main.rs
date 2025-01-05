fn main() {
    dbg!(Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]));
}

struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for idx in 0..nums.len() {
            let n = nums[idx];
            if idx % 2 == 0 && n % 2 == 1 || idx % 2 == 1 && n % 2 == 0 {
                let mut next = idx + 1;
                while nums[next] % 2 != idx as i32 % 2 {
                    next += 1;
                }
                nums.swap(idx, next);
            }
        }
        nums
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sort_array_by_parity_ii() {
        assert_eq!(
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            vec![4, 5, 2, 7]
        );
    }
}
