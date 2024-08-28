fn main() {
    let a: Vec<String> = Solution::letter_combinations(String::from("23"));
    let b: Vec<String> =
        stringvec::stringvec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
    dbg!(a == b);
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn backtrack(
            digits: &Vec<char>,
            idx: usize,
            length: usize,
            letters: &mut Vec<char>,
            result: &mut Vec<String>,
            m: &HashMap<char, Vec<char>>,
        ) {
            if length == 0 {
                return;
            }

            if idx == length {
                result.push(letters.iter().collect());
                return;
            }

            m.get(&digits[idx]).unwrap().iter().for_each(|d: &char| {
                letters.push(*d);
                backtrack(digits, idx + 1, length, letters, result, m);
                letters.pop();
            });
        }

        let m: HashMap<char, Vec<char>> = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        let mut result: Vec<String> = vec![];
        let digits: Vec<char> = digits.chars().collect();
        let length: usize = digits.len();
        let mut letters: Vec<char> = vec![];

        backtrack(&digits, 0, length, &mut letters, &mut result, &m);

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use stringvec::stringvec;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            stringvec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
