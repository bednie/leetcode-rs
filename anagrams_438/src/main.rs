fn main() {
    dbg!(Solution::find_anagrams(
        "cbaebabacd".to_string(),
        "abc".to_string()
    ));
}

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut p_freqs = vec![0; 26];
        let mut s_freqs = vec![0; 26];
        let mut result = vec![];
        p.bytes().for_each(|c| p_freqs[(c - b'a') as usize] += 1);
        //println!("{:?}", p_freqs);

        for (i, c) in s.bytes().enumerate() {
            s_freqs[(c - b'a') as usize] += 1;

            if i >= p.len() {
                s_freqs[(s.as_bytes()[i - p.len()] - b'a') as usize] -= 1;
            }

            //println!("{:?}", s_freqs);

            if p_freqs == s_freqs {
                result.push((i + 1 - p.len()) as i32);
            }
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_anagrams() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
    }
}
