fn main() {
    dbg!(Solution::find_different_binary_string(vec![
        "111".to_string(),
        "011".into(),
        "001".into()
    ]));
}

struct Solution;

/*
use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let width = nums.len();
        let max_value: u32 = 1 << width;
        let mut appears: HashSet<u32> = nums.iter().map(|n| u32::from_str_radix(&n, 2).unwrap()).collect();
        let mut ans = "".to_string();

        for num in 0..max_value {
            if !appears.contains(&num) {
                ans = format!("{:0width$b}", num);
                break;
            }
        }

        ans
    }
}
*/

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        (0..nums.len())
            .map(|idx| {
                if nums[idx].as_bytes()[idx] == b'0' {
                    '1'
                } else {
                    '0'
                }
            })
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_different_binary_string() {
        assert_eq!(
            Solution::find_different_binary_string(vec![
                "111".to_string(),
                "011".into(),
                "001".into()
            ]),
            "000".to_string()
        );
    }
}
