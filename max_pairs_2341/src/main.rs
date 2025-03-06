fn main() {
    dbg!(Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]));
}

struct Solution;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0, 0];
        let mut nums = nums;
        nums.sort_unstable();
        nums.push(-1);

        let mut idx = 0;
        while idx < nums.len() - 1 {
            if nums[idx] == nums[idx + 1] {
                result[0] += 1;
                idx += 2;
            } else {
                result[1] += 1;
                idx += 1;
            }
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_number_of_pairs() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
            vec![3, 1]
        );
    }
}
