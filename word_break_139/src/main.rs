fn main() {
    dbg!(Solution::word_break(
        String::from("leetcode"),
        vec![String::from("leet"), String::from("code")]
    ));
}

struct Solution;

// Port of ayush-1's solution:
// https://leetcode.com/problems/word-break/solutions/5272806/efficient-0ms-word-break-solution-using-trie-in-java-c-and-python-beats-100

#[derive(Default, Debug)]
struct Trie {
    is_end: bool,
    next: [Option<Box<Trie>>; 27],
}

impl Trie {
    #[inline(always)]
    fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    fn insert(&mut self, word: String) {
        let mut current = self;

        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            current = current.next[index].get_or_insert_with(|| Box::new(Trie::new()));
        }
        current.is_end = true;
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut trie = Trie::new();
        for w in word_dict.into_iter() {
            trie.insert(w.clone());
        }

        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 0..s.len() {
            if !dp[i] {
                continue;
            }

            let mut node = &trie; // get root of trie

            for j in i..s.len() {
                let index = (s[j] as u8 - b'a') as usize;

                if let Some(next) = &node.next[index] {
                    node = next;
                    if node.is_end {
                        dp[j + 1] = true;
                    }
                } else {
                    break;
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(Solution::word_break(
            String::from("leetcode"),
            vec![String::from("leet"), String::from("code")]
        ));
    }
}
