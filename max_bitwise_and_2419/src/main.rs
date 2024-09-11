use std::cmp::Ordering;

fn main() {
    dbg!(Solution::longest_subarray(vec![
        96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979
    ]));
}

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_element = nums[0];
        let mut max_len = 1;
        let mut end = 0;

        while end < nums.len() {
            match nums[end].cmp(&max_element) {
                Ordering::Less => end += 1,
                Ordering::Equal => {
                    let start = end;
                    while end < nums.len() && nums[end] == nums[start] {
                        end += 1;
                    }
                    max_len = max_len.max(end - start);
                },
                Ordering::Greater => {
                    max_element = nums[end];
                    max_len = 1;
                },
            }
        }
        max_len as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_count_consistent_strings() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);

        assert_eq!(
            Solution::longest_subarray(vec![
                96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979
            ]),
            1
        );
    }
}
