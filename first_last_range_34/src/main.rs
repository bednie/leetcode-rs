fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn search_left(nums: Vec<i32>, target: i32, right: i32) -> i32 {
            for i in (0..=right).rev() {
                if nums[i as usize] != target {
                    return i + 1;
                }
            }
            0_i32
        }

        fn search_right(nums: Vec<i32>, target: i32, left: i32) -> i32 {
            for i in left..nums.len() as i32 {
                if nums[i as usize] != target {
                    return i - 1;
                }
            }
            nums.len() as i32 - 1
        }

        fn binary_search(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut left = 0;
            let mut right = nums.len() - 1;
            let mut mid: usize;
            while left <= right {
                mid = (right + left) / 2;

                if nums[mid as usize] < target {
                    left = mid + 1;
                } else if nums[mid as usize] > target {
                    right = mid - 1;
                } else {
                    return vec![
                        search_left(nums.clone(), target, mid as i32),
                        search_right(nums.clone(), target, mid as i32),
                    ];
                }
            }
            vec![-1_i32, -1_i32]
        }

        if nums.is_empty() || nums.len() == 1 && nums[0] != target {
            return vec![-1, -1];
        }

        let mut output = vec![-1_i32; 2];

        if let Some(x) = nums.first() {
            if *x == target {
                output[0] = 0;
            }
        }

        if let Some(x) = nums.last() {
            if *x == target {
                output[1] = nums.len() as i32 - 1;
            }
        }

        match (output[0], output[1]) {
            (-1, 0..=100_000) => {
                vec![search_left(nums, target, output[1]), output[1]]
            }
            (0..=100_000, -1) => {
                vec![output[0], search_right(nums, target, output[0])]
            }
            (0..=100_000, 0..=100_000) => output,
            (-1, -1) => {
                if nums.len() == 2 {
                    output
                } else {
                    binary_search(nums, target)
                }
            }
            (_, _) => {
                panic!("Something went wrong!");
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_search_range() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3, 4]);
    }
}