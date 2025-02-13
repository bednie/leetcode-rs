fn main() {
    dbg!(Solution::count_characters(
        vec![
            "cat".to_string(),
            "bt".to_string(),
            "hat".to_string(),
            "tree".to_string()
        ],
        "atach".to_string()
    ));
}

struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let chars_freq = chars.chars().fold([0; 26], |mut acc, c| {
            acc[c as usize - 97] += 1;
            acc
        });
        let mut count = 0;

        for word in words {
            let word_freq = word.chars().fold([0; 26], |mut acc, c| {
                acc[c as usize - 97] += 1;
                acc
            });

            if std::iter::zip(&chars_freq, &word_freq).all(|(c, w)| c >= w) {
                count += word.len();
            }
        }

        count as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_count_characters() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            ),
            6
        );
    }
}
