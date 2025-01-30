fn main() {
    dbg!(Solution::common_chars(vec![
        "bella".to_string(),
        "label".to_string(),
        "roller".to_string()
    ]),);
}

struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut common = vec![(0, 0); 26]; // (count words, min occurences)

        for word in words.iter() {
            let mut seen = [0; 26];
            for w in word.bytes() {
                seen[w as usize - 97] += 1;
            }

            for idx in 0..26 {
                if seen[idx] > 0 {
                    common[idx] = (
                        common[idx].0 + 1,
                        if common[idx].1 == 0 {
                            seen[idx]
                        } else {
                            common[idx].1.min(seen[idx])
                        },
                    );
                }
            }
        }

        let mut ans = vec![];
        for (c, counts) in common.iter().enumerate() {
            if counts.0 == words.len() {
                for _ in 0..counts.1 {
                    ans.push(((c as u8 + 97) as char).to_string());
                }
            }
        }
        ans
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_common_chars() {
        assert_eq!(
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            vec!["e".to_string(), "l".to_string(), "l".to_string()]
        );
    }
}
