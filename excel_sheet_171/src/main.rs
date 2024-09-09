fn main() {
    dbg!(Solution::title_to_number(String::from("FXSHRXW")));
}

struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .fold(0, |acc: i32, c: char| acc * 26 + (c as u8 - b'@') as i32)
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
