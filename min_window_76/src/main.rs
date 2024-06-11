fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    // with help from stanislav-iablokov:
    //  https://leetcode.com/problems/minimum-window-substring/solutions/2732150/python-rust-c-fast-concise-using-two-pointer-sliding-window-with-detailed-comments
    pub fn min_window(s: String, t: String) -> String {
        let mut ans = String::from("");
        let mut freqs = std::collections::HashMap::new();
        let mut s_chars = s.chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, 0);
        let (mut min_left, mut min_right) = (0, s_chars.len() + 1);

        for c in t.chars() {
            *freqs.entry(c).or_insert(0) += 1;
        }

        let mut count = freqs.len() as i32;

        loop {
            if right < s_chars.len() && count > 0 {
                if let Some(x) = freqs.get_mut(&s_chars[right]) {
                    *x -= 1;
                    if *x == 0 {
                        count -= 1;
                    }
                }
                right += 1;
            } else if left < s_chars.len() && count <= 0 {
                if right - left < min_right - min_left {
                    min_right = right;
                    min_left = left;
                }

                if let Some(x) = freqs.get_mut(&s_chars[left]) {
                    if *x == 0 {
                        count += 1;
                    }
                    *x += 1;
                }
                left += 1;
            } else {
                break;
            }
        }

        if min_right <= s_chars.len() {
            ans = s[min_left..min_right].to_string();
        }
        ans
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")),
            String::from("BANC")
        );
    }
}
