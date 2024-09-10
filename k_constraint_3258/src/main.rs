fn main() {
    dbg!(Solution::count_k_constraint_substrings(
        "11111".to_string(),
        1
    ));
}

struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut zeros = 0_i32;
        let mut ones = 0_i32;
        let mut fast = 0_usize;
        let mut slow = 0_usize;
        let mut count = 0_usize;
        let s: &[u8] = s.as_bytes();

        while fast < s.len() {
            if s[fast] == b'0' {
                zeros += 1;
            } else {
                ones += 1;
            }

            while zeros > k && ones > k {
                if s[slow] == b'0' {
                    zeros -= 1;
                } else {
                    ones -= 1;
                }
                slow += 1;
            }

            count += fast - slow + 1;
            fast += 1;
        }
        count as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_count_k_constraint_substrings() {
        assert_eq!(
            Solution::count_k_constraint_substrings("11111".to_string(), 1),
            15
        );
    }
}
