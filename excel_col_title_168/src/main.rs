fn main() {
    dbg!(Solution::convert_to_title(i32::MAX));
}

struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let mut title: Vec<char> = vec![];

        while column_number > 0 {
            column_number -= 1;
            title.push(((column_number % 26) as u8 + b'A') as char);
            column_number /= 26;
        }
        String::from_iter(title.into_iter().rev())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(
            Solution::convert_to_title(i32::MAX),
            String::from("FXSHRXW")
        );
    }
}
