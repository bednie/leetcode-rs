fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        // I developed various solutions which were too slow.
        // Referenced nandan_bhamdary's solution for help:
        // https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/5553426/kadane-s-algorithm-maxsubarraysumcircular-in-1-pass-o-n-optimal

        let mut max_curr = nums[0];
        let mut max_subarray = nums[0];

        let mut min_curr = nums[0];
        let mut min_subarray = nums[0];

        let mut sum = nums[0];

        for i in nums.into_iter().skip(1) {
            max_curr = std::cmp::max(max_curr + i, i);
            max_subarray = std::cmp::max(max_subarray, max_curr);

            min_curr = std::cmp::min(min_curr + i, i);
            min_subarray = std::cmp::min(min_subarray, min_curr);

            sum += i;
        }

        if min_subarray == sum {
            return max_subarray;
        }
        std::cmp::max(max_subarray, sum - min_subarray)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_max_subarray_sum_circular() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    }
}
