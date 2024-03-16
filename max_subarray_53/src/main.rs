struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // panic on empty array
        if nums.is_empty() {
            panic!()
        }

        // early return for trivial cases
        if nums.len() == 1 {
            return nums[0];
        }

        //
        fn max_crossing(nums: &[i32], middle: usize, low: usize, high: usize) -> i32 {
            // left
            let mut left_current_sum: i32 = 0;
            let mut left: i32 = -10001; // initialize to be less than leetcode constraint n[i] >= pow(-10,4)
            for i in (low..=middle).rev() {
                left_current_sum += nums[i];
                // update max
                if left_current_sum > left {
                    left = left_current_sum;
                }
            }

            // loop through right array from middle to end
            let mut right_current_sum: i32 = 0;
            let mut right: i32 = -10001; // initialize to be less than leetcode constraint n[i] >= pow(-10,4)
            (middle + 1..=high).for_each(|i| {
                right_current_sum += nums[i];
                // update max
                if right_current_sum > right {
                    right = right_current_sum;
                }
            });

            left + right //- nums[middle]
        }

        fn max_sub_array_recursive(nums: &[i32], low: usize, high: usize) -> i32 {
            // base case: array is of length 1
            if low == high {
                return nums[low];
            }

            let middle_index = (low + high) / 2;

            // divide-and-conquer
            let left_max = max_sub_array_recursive(nums, low, middle_index);
            let right_max = max_sub_array_recursive(nums, middle_index + 1, high);
            let cross_max = max_crossing(nums, middle_index, low, high);

            vec![left_max, right_max, cross_max]
                .into_iter()
                .max()
                .unwrap()
        }

        max_sub_array_recursive(&nums, 0, nums.len() - 1)
    }
}

fn main() {
    let nums = vec![
        84, 87, 78, 16, 94, 36, 87, 93, 50, 22, 63, 28, 91, 60, 64, 27, 41, 27, 73, 37, 12, 69, 68,
        30, 83, 31, 63, 24, 68, 36, 30, 3, 23, 59, 70, 68, 94, 57, 12, 43, 30, 74, 22, 20, 85, 38,
        99, 25, 16, 71, 14, 27, 92, 81, 57, 74, 63, 71, 97, 82, 6, 26, 85, 28, 37, 6, 47, 30, 14,
        58, 25, 96, 83, 46, 15, 68, 35, 65, 44, 51, 88, 9, 77, 79, 89, 85, 4, 52, 55, 100, 33, 61,
        77, 69, 40, 13, 27, 87, 95, 40,
    ];

    let max_subarray = Solution::max_sub_array(nums);
    println!("{:?}", max_subarray)
}

#[cfg(test)]
mod test {
    use super::*;
    // tests
    #[test]
    fn test_valid_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_trivial_array() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    #[should_panic]
    fn test_empty_array() {
        Solution::max_sub_array(vec![]);
    }

    #[test]
    fn test_larger_array_all_neg_values() {
        assert_eq!(
            Solution::max_sub_array(vec![
                -1, -84, -87, -78, -16, -94, -36, -87, -93, -50, -22, -63, -28, -91, -60, -64, -27,
                -41, -27, -73, -37, -12, -69, -68, -30, -83, -31, -63, -24, -68, -36, -30, -3, -23,
                -59, -70, -68, -94, -57, -12, -43, -30, -74, -22, -20, -85, -38, -99, -25, -16,
                -71, -14, -27, -92, -81, -57, -74, -63, -71, -97, -82, -6, -26, -85, -28, -37, -6,
                -47, -30, -14, -58, -25, -96, -83, -46, -15, -68, -35, -65, -44, -51, -88, -9, -77,
                -79, -89, -85, -4, -52, -55, -100, -33, -61, -77, -69, -40, -13, -27, -87, -95,
                -40
            ]),
            -1
        );
    }

    #[test]
    fn test_larger_array() {
        assert_eq!(
            Solution::max_sub_array(vec![
                84, 87, 78, 16, 94, 36, 87, 93, 50, 22, 63, 28, 91, 60, 64, 27, 41, 27, 73, 37, 12,
                69, 68, 30, 83, 31, 63, 24, 68, 36, 30, 3, 23, 59, 70, 68, 94, 57, 12, 43, 30, 74,
                22, 20, 85, 38, 99, 25, 16, 71, 14, 27, 92, 81, 57, 74, 63, 71, 97, 82, 6, 26, 85,
                28, 37, 6, 47, 30, 14, 58, 25, 96, 83, 46, 15, 68, 35, 65, 44, 51, 88, 9, 77, 79,
                89, 85, 4, 52, 55, 100, 33, 61, 77, 69, 40, 13, 27, 87, 95, 40
            ]),
            5284
        );
    }
}
