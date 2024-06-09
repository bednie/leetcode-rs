fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (right + left) / 2;
            if nums[mid as usize] < nums[mid as usize + 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_peak_element() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    }
}
