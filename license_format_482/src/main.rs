fn main() {
    dbg!(Solution::license_key_formatting(
        String::from("5F3Z-2e-9-w"),
        4
    ));
}

struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s_stripped = s.chars().filter(|c| *c != '-').collect::<String>();
        let mut license_key = String::new();
        let first = s_stripped.len() % k as usize; // first chunk

        license_key.push_str(&s_stripped[..first].to_string().to_uppercase());

        for i in 0..(s_stripped.len() / k as usize) {
            if !license_key.is_empty() {
                license_key.push('-');
            }
            license_key.push_str(
                &s_stripped[i * k as usize + first..i * k as usize + k as usize + first]
                    .to_uppercase(),
            );
        }
        license_key
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_license_key_formatting() {
        assert_eq!(
            Solution::license_key_formatting(String::from("5F3Z-2e-9-w"), 4),
            String::from("5F3Z-2E9W")
        );
    }
}
