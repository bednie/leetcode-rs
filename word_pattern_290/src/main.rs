use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s: Vec<_> = s.split_whitespace().collect();

        if s.len() != pattern.len() {
            return false;
        } else {
            let mut map: HashMap<char, &str> = HashMap::new();
            let mut pam: HashMap<&str, char> = HashMap::new();
            for (i, c) in pattern.chars().enumerate() {
                if (map.contains_key(&c) && map.get(&c) != Some(&s[i]))
                    || (pam.contains_key(&s[i]) && pam.get(&s[i]) != Some(&c))
                {
                    return false;
                } else {
                    map.insert(c, s[i]);
                    pam.insert(s[i], c);
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_array() {
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog")),
            true
        );
    }
}
