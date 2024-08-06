fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // *nums.iter().min().unwrap() -- no trolling
        let (mut left, mut right, mut mid) = (0, nums.len() - 1, 0);
        while nums[left] > nums[right] {
            mid = (left + right) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return nums[left];
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![2, 3, 4, 5, 1]), 1);
    }
}
