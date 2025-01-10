fn main() {
    dbg!(Solution::most_common_word(
        "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
        vec!["hit".to_string()]
    ));
}

struct Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned: std::collections::HashSet<String> = banned.into_iter().collect();

        let p: Vec<String> = paragraph
            .split([' ', '!', '?', '\'', ',', ';', '.'])
            .map(|w| w.to_lowercase())
            .filter(|w| !w.is_empty() && !banned.contains(w))
            .collect();

        let mut f = std::collections::HashMap::new();

        p.into_iter().for_each(|word| {
            f.entry(word)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        f.into_iter()
            .max_by_key(|word| word.1)
            .unwrap()
            .0
            .to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_most_common_word() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball".to_string()
        );
    }
}
