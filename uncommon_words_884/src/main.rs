use std::collections::HashSet;

fn main() {
    dbg!(Solution::uncommon_from_sentences(
        String::from("this apple is sweet"),
        String::from("this apple is sour")
    ));
}

struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut p1 = HashSet::new();
        let mut p2 = HashSet::new();
        let mut dupes = HashSet::new();

        for s in s1.split(' ').collect::<Vec<&str>>().into_iter() {
            if p1.contains(s) {
                dupes.insert(s.to_string());
            } else {
                p1.insert(s.to_string());
            }
        }

        for s in s2.split(' ').collect::<Vec<&str>>().into_iter() {
            if p2.contains(s) {
                dupes.insert(s.to_string());
            } else {
                p2.insert(s.to_string());
            }
        }

        for s in dupes.into_iter() {
            if p1.contains(&s) {
                p1.remove(&s);
            }

            if p2.contains(&s) {
                p2.remove(&s);
            }
        }

        p1.symmetric_difference(&p2)
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use stringvec::stringvec;

    #[test]
    fn test_uncommon_from_sentences() {
        let result = Solution::uncommon_from_sentences(
            String::from("this apple is sweet"),
            String::from("this apple is sour"),
        );
        assert!(result == stringvec!["sour", "sweet"] || result == stringvec!["sweet", "sour"]);
    }
}
