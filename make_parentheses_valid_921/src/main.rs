fn main() {
    dbg!(Solution::min_add_to_make_valid(String::from("()))((")));
}

struct Solution;


impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut unclosed: i32 = 0;
        let mut unopened: i32 = 0;

        for c in s.bytes() {
            if c == b'(' {
                unclosed += 1;
            } else if unclosed > 0 {
                unclosed -= 1;
            } else {
                unopened += 1;
            }
        }
        unclosed + unopened
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_add_to_make_valid() {
        assert_eq!(Solution::min_add_to_make_valid(String::from("()))((")), 4);
    }
}
