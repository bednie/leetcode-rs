fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut v_s = vec![0; 26];
        
        for c in s.chars() {
            v_s[c as usize - 97] += 1;
        }

        for c in t.chars() {
            v_s[c as usize - 97] -= 1;
        }

        for i in v_s.into_iter() {
            if i != 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert!(Solution::is_anagram(String::from("anagram"), String::from("nagaram")));
    }
}

