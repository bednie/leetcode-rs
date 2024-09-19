fn main() {
    dbg!(Solution::wonderful_substrings(String::from("aabb")));
}

struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut mask = 0_usize;
        let mut count = [0_usize; 1024]; // 2**10 possible states
        let mut result = 0_usize;
        count[0] = 1; // empty substring is wonderful

        word.as_bytes().iter().for_each(|w| {
            mask ^= 1 << (w - b'a');
            result += count[mask];
            count[mask] += 1;

            // a through j
            (0..10).for_each(|c| {
                // states with unseen chars add 0
                result += count[mask ^ (1 << c)]
            });
        });
        result as i64
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_wonderful_substrings() {
        assert_eq!(Solution::wonderful_substrings(String::from("aabb")), 9);
    }
}
