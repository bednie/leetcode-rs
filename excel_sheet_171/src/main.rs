fn main() {
    dbg!(Solution::title_to_number(String::from("FXSHRXW")));
}

struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .as_bytes()
            .iter()
            .fold(0, |acc: i32, c: &u8| acc * 26 + (c - b'@') as i32)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(Solution::title_to_number(String::from("FXSHRXW")), i32::MAX);
    }
}
