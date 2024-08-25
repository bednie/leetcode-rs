use core::str;

fn main() { 
    dbg!(Solution::generate_parenthesis(3));
}

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(pairs: &str, opened: u8, closed: u8, result: &mut Vec<String>, n: u8) {
            if pairs.len() as u8 == n * 2 {
                result.push(String::from(pairs));
                return;
            }

            if opened < n {
                backtrack(&[pairs, "("].concat(), opened + 1, closed, result, n);
            }

            if closed < opened {
                backtrack(&[pairs, ")"].concat(), opened, closed + 1, result, n);
            }
        }

        let mut result: Vec<String> = vec![];
        let pair: &str = "(";
        backtrack(pair, 1, 0, &mut result, n as u8);
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let pairs: Vec<String> = vec![
            "((()))".to_string(),
            "(()())".into(),
            "(())()".into(),
            "()(())".into(),
            "()()()".into(),
        ];
        assert_eq!(Solution::generate_parenthesis(3), pairs);
    }
}
