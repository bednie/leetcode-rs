fn main() {
    dbg!(Solution::dest_city(vec![
        vec!["London".to_string(), "New York".to_string()],
        vec!["New York".to_string(), "Lima".to_string()],
        vec!["Lima".to_string(), "Sao Paulo".to_string()]
    ]));
}

struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let m: std::collections::HashMap<&str, &str> = paths
            .iter()
            .map(|path| (&path[0][..], &path[1][..]))
            .collect();

        m.values()
            .find(|&dest| !m.contains_key(dest))
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_dest_city() {
        assert_eq!(
            Solution::dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ]),
            "Sao Paulo".to_string()
        );
    }
}
