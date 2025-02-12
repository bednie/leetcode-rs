fn main() {
    dbg!(Solution::print_vertically("HOW ARE YOU".to_string()));
}

struct Solution;

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut result = vec![];
        let s: Vec<&str> = s.split(" ").collect();
        let max_len = s.iter().fold(0, |acc, n| acc.max(n.len()));

        for col in 0..max_len {
            let mut part = "".to_string();

            for word in &s {
                if col < word.len() {
                    part.push_str(&word[col..col + 1]);
                } else {
                    part.push(' ');
                }
            }
            result.push(part.trim_end().to_string());
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_print_vertically() {
        assert_eq!(
            Solution::print_vertically("HOW ARE YOU".to_string()),
            vec!["HAY".to_string(), "ORO".to_string(), "WEU".to_string()]
        );
    }
}
