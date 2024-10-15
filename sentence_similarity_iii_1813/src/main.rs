fn main() {
    dbg!(Solution::are_sentences_similar(
        "My name is Haley".to_string(),
        "My Haley".to_string()
    ));
}

struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence1 == sentence2 {
            return true;
        }

        let mut s1 = sentence1.split(" ").collect::<Vec<&str>>();
        let mut s2 = sentence2.split(" ").collect::<Vec<&str>>();

        while !s1.is_empty() && !s2.is_empty() && s1[0] == s2[0] {
            s1.remove(0);
            s2.remove(0);
        }

        while !s1.is_empty() && !s2.is_empty() && s1[s1.len() - 1] == s2[s2.len() - 1] {
            s1.pop();
            s2.pop();
        }

        if s1.is_empty() || s2.is_empty() {
            return true;
        }
        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_are_sentences_similar() {
        assert!(!Solution::are_sentences_similar(
            "My name is Haley".to_string(),
            "name is".to_string()
        ));

        assert!(Solution::are_sentences_similar(
            "My name is Haley".to_string(),
            "My Haley".to_string()
        ));
    }
}
