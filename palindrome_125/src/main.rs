fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_chars: Vec<char> = s.to_lowercase().chars().collect();
        let (mut i, mut j) = (0, s_chars.len() - 1);
        while i <= j && i < s_chars.len() && i < s_chars.len()  {
            if !s_chars[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            if !s_chars[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if s_chars[i] != s_chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(String::from("Racecar")));
    }
}
