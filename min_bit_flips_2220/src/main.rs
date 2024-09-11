fn main() {
    dbg!(Solution::min_bit_flips(10, 7));
}

struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_bit_flips() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
    }
}
