fn main() {
    dbg!(Solution::convert_date_to_binary("2080-02-29".to_string()));
}

struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let y = &date[..4].parse::<usize>().unwrap();
        let m = &date[5..7].parse::<usize>().unwrap();
        let d = &date[8..].parse::<usize>().unwrap();
        format!("{y:b}-{m:b}-{d:b}")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_convert_date_to_binary() {
        assert_eq!(
            Solution::convert_date_to_binary("2080-02-29".to_string()),
            "100000100000-10-11101".to_string()
        );
    }
}