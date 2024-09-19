fn main() {
    dbg!(Solution::convert_date_to_binary("2080-02-29".to_string()));
}

struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        match date[..4].parse::<u16>() {
            Ok(year) => {
                match date[5..7].parse::<u8>() {
                    Ok(month) => {
                        match date[8..].parse::<u8>() {
                            Ok(day) => format!("{year:b}-{month:b}-{day:b}"),
                            Err(_e) => format!("{_e}")
                        }
                    }
                    Err(_e) => format!("{_e}")
                }   
            }
            Err(_e) => format!("{_e}")
        }
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