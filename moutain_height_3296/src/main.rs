fn main() {
    dbg!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]));
}

struct Solution;

// Formula help from user SalvadorDali:
// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/solutions/5819062/python-rust-binary-search-with-explanation

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        fn can_zero_height(seconds: i64, height: i32, worker_times: &[i32]) -> bool {
            worker_times.iter().fold(0, |acc, &worker| {
                acc + ((f64::sqrt(4.0 * (2.0 * seconds as f64 / worker as f64) + 1.0) - 1.0) / 2.0)
                    .floor() as i64
            }) >= height as i64
        }

        let mut left: i64 = 1;
        let mut right: i64 = i64::MAX / 2;
        while left < right {
            let mid = (left + right) / 2;
            if can_zero_height(mid, mountain_height, &worker_times) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_number_of_seconds() {
        assert_eq!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]), 12)
    }
}
