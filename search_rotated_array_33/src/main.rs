fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    // Initially I wrote a solution (see Solution2) 
    // to find the pivot and then binary search the 
    // partition whose range contains the target, 
    // but this solution from phistellar is a lot simpler:
    // https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3879413/rust-go-python-binary-search
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] == target {
                return left as i32;
            }

            if nums[right] == target {
                return right as i32;
            }

            // left half is sorted
            if nums[mid] >= nums[left] {
                // target is in range
                if nums[mid] > target && nums[left] <= target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }

            // right half is sorted
            } else {
                // target is in range
                if nums[mid] < target && nums[right] >= target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}

struct Solution2;

impl Solution2 {
    pub fn find_min(nums: &Vec<i32>) -> usize {
        let (mut left, mut right, mut mid) = (0, nums.len() - 1, 0);
        while nums[left] > nums[right] {
            mid = (left + right) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left;
    }

    pub fn binary_search(mut left: usize, mut right: usize, nums: &Vec<i32>, target: i32) -> i32 {
        while left <= right {
            let mid = (left + right) / 2;

            if mid > nums.len() {
                return -1;
            }

            if nums[mid] == target {
                return mid as i32;
            }

            if target >= nums[left] && target < nums[mid] {
                return Self::binary_search(left, mid - 1, nums, target);
            } else if target > nums[mid] && target <= nums[right] {
                return Self::binary_search(mid + 1, right, nums, target);
            } else {
                break;
            }
        }
        -1
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = Self::find_min(&nums);
        let (mut left, mut right) = (-1, -1);

        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            } else {
                return -1;
            }
        }

        left = Self::binary_search(0, pivot - 1, &nums, target);
        right = Self::binary_search(pivot, nums.len() - 1, &nums, target);

        match (left > -1, right > -1) {
            (true, false) => return left,
            (false, true) => return right,
            (true, true) => return left,
            (false, false) => return -1,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_search2() {
        assert_eq!(Solution2::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }
}
