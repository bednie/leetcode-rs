use std::collections::HashSet;
use stringvec::stringvec as sv;

fn main() {
    dbg!(Solution::count_consistent_strings(
        "cad".to_string(),
        sv!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]
    ));
}

struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed: HashSet<char> = allowed.chars().collect();
        let mut result = 0;

        'outer: for word in words {
            for w in word.chars() {
                if !allowed.contains(&w) {
                    continue 'outer;
                }
            }
            result += 1;
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_count_consistent_strings() {
        assert_eq!(
            Solution::count_consistent_strings(
                "cad".to_string(),
                sv!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]
            ),
            4
        );
    }
}
