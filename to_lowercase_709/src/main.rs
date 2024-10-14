fn main() {
    dbg!(Solution::to_lower_case("al&phaBET".to_string()));
}

struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        String::from_utf8(
            s.bytes()
                .map(|c| {
                    if c >= b'A' && c <= b'Z' {
                        c + (b'a' - b'A')
                    } else {
                        c
                    }
                })
                .collect(),
        )
        .unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_to_lowercase() {
        assert_eq!(
            Solution::to_lower_case("al&phaBET".to_string()),
            "al&phabet".to_string()
        );
    }
}
