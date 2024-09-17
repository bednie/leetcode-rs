fn main() {
    dbg!(Solution::string_hash(String::from("abcd"), 2));
}

struct Solution;

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut result: String = String::from("");
        s.as_bytes().chunks(k as usize).for_each(|s: &[u8]| {
            let mut h: u32 = 0;
            s.iter().for_each(|c: &u8| {
                h += (c - b'a') as u32;
            });
            result.push(char::from_u32(h % 26 + b'a' as u32).unwrap());
        });
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
