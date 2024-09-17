fn main() {
    dbg!(Solution::string_hash(String::from("abcd"), 2));
}

struct Solution;

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut result: String = String::from("");
        let mut s: std::slice::Chunks<'_, u8> = s.as_bytes().chunks(k as usize);

        while let Some(chunk) = s.next() {
            let mut h: u32 = 0;
            for c in chunk.iter() {
                h += (c - b'a') as u32;
            }
            result.push(char::from_u32(h % 26 + b'a' as u32).unwrap());
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
