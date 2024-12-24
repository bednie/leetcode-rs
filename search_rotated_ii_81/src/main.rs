fn main() {
    dbg!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
}

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }

        let mut low = 0;
        let mut high = nums.len() - 1;

        while low <= high {
            if nums[low] == target || nums[high] == target {
                return true;
            }
            low += 1;
            high -= 1;
        }
        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_search() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
