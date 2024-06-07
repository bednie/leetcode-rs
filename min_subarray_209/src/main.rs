fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_length = 10_usize.pow(5) + 1;
        let mut sum = nums[0];
        let mut i = 0;
        let mut j = 1;

        while i < nums.len() || j < nums.len() {
            if sum >= target {
                min_length = std::cmp::min(min_length, nums[i..j].len());
                sum -= nums[i];
                i += 1;
            }
            if sum < target {
                if j < nums.len() {
                    sum += nums[j];
                    j += 1;
                } else if i < nums.len() {
                    sum -= nums[i];
                    i += 1;
                } else {
                    break;
                }
            }
        }
        if min_length == 100_001 {
            return 0;
        }
        min_length as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }
}
