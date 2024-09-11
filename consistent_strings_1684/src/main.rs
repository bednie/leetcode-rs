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
        let allowed: usize = allowed
            .chars()
            .fold(0, |acc, c| acc | 1 << (c as u8 - b'a'));

        words.iter().fold(0, |count, word| {
            if allowed | word.chars().fold(0, |acc, c| acc | 1 << (c as u8 - b'a')) <= allowed {
                count + 1
            } else {
                count
            }
        })
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
