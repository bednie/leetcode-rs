fn main() {
    dbg!(Solution::string_hash(String::from("abcd"), 2));
}

struct Solution;

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut result = String::from("");
        let mut start = 0 as usize;
        let mut end = k as usize;

        while end <= s.len() {
            let mut h = 0_u32;
            let sub = &s[start..end];
            for c in sub.chars() {
                h += c as u32 - b'a' as u32;
            }

            result.push((h % 26 + b'a' as u32) as u8 as char);
            start = end;
            end += k as usize;
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(
            Solution::string_hash(String::from("abcd"), 2),
            String::from("bf")
        );
    }
}
