use std::time::{Duration, Instant};

fn main() { 
    let start: Instant = Instant::now();
    let n = 15; 
    Solution::generate_parenthesis(n);
    let duration: Duration = start.elapsed();
    println!("Time elapsed to generate {} pairs of () is: {:?}", n, duration);
}

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(pairs: &mut String, opened: u8, closed: u8, result: &mut Vec<String>, n: u8) {
            if opened + closed == pairs.capacity() as u8 {
                result.push(pairs.clone());
                return;
            }

            if opened < n {
                pairs.push('(');
                backtrack(pairs, opened + 1, closed, result, n);
                pairs.pop();
            }

            if closed < opened {
                pairs.push(')');
                backtrack(pairs, opened, closed + 1, result, n);
                pairs.pop();
            }
        }

        let mut result: Vec<String> = vec![];
        let mut pairs = String::with_capacity((n*2) as usize);
        pairs.push('(');
        backtrack(&mut pairs, 1, 0, &mut result, n as u8);
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
