fn main() {
    let begin_word = String::from("hit");
    let end_word = String::from("cog");
    let word_list = Vec::from([
        "hot".to_string(),
        "dot".into(),
        "dog".into(),
        "lot".into(),
        "log".into(),
        "cog".into(),
    ]);
    let ladder_length = Solution::ladder_length(begin_word, end_word, word_list);
    dbg!(ladder_length);
}

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut words = std::collections::HashSet::new();

        for word in word_list.iter() {
            words.insert(word);
        }

        if !words.contains(&end_word) {
            return 0;
        }

        let mut deque = std::collections::VecDeque::from([(1, begin_word)]);

        while let Some((transformations, word)) = deque.pop_front() {
            for i in 0..end_word.len() {
                for c in b'a'..=b'z' {
                    let candidate = String::from(
                        [&word[..i], (c as char).to_string().as_str(), &word[i + 1..]].concat(),
                    );

                    if candidate == end_word {
                        return transformations + 1;
                    }

                    if words.contains(&candidate) {
                        words.remove(&candidate);
                        deque.push_back((transformations + 1, candidate));
                    }
                }
            }
        }
        0
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_ladder_length() {
        let begin_word = String::from("hit");
        let end_word = String::from("cog");
        let word_list = Vec::from([
            "hot".to_string(),
            "dot".into(),
            "dog".into(),
            "lot".into(),
            "log".into(),
            "cog".into(),
        ]);

        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
    }
}
