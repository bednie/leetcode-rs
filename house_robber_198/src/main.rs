fn main() {
    dbg!(Solution::rob(vec![2,7,9,3,1]));
}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; 128];
        for (idx, amount) in nums.into_iter().enumerate() {
            dp[idx + 2] = dp[idx + 2].max(dp[idx] + amount);
            dp[idx + 1] = dp[idx + 1].max(dp[idx]);
        }
        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![2,7,9,3,1]), 12);
    }
}