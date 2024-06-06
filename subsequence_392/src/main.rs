fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        if t.len() == 0 {
            return false;
        }

        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let (mut s_idx, mut t_idx) = (0_usize, 0_usize);

        while t_idx < t.len() {
            if t[t_idx] == s[s_idx] {
                s_idx += 1;
            }
            if s_idx >= s.len() {
                return true;
            }
            t_idx += 1;
        }
        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        assert!(Solution::is_subsequence(
            String::from("abc"),
            String::from("ahbgdc")
        ));
    }
}
