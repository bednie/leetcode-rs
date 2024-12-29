fn main() {
    dbg!(
        Solution::maximum_product(vec![-1,-2,-3]),
        Solution::maximum_product(vec![2,1,3,4]),
        Solution::maximum_product(vec![3,-1,-2,1,2])
    );
}

struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[0..3].iter().product::<i32>().max(nums[0..2].iter().product::<i32>() * nums[nums.len()-1]).max(nums[0] * nums[nums.len()-2..].iter().product::<i32>()).max(nums[(nums.len() as i32 - 3).max(0) as usize..].iter().product::<i32>())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_maximum_product() {
        assert_eq!(Solution::maximum_product(vec![-1,-2,-3]), -6);
        assert_eq!(Solution::maximum_product(vec![2,1,3,4]), 24);
        assert_eq!(Solution::maximum_product(vec![3,-1,-2,1,2]), 6);
    }
}