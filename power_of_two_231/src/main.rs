fn main() {
    dbg!(Solution::is_power_of_two(16));
}

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n.leading_zeros() > 0 && n.count_ones() == 1
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert!(Solution::is_power_of_two(16));
    }

    #[test]
    fn test_is_not_power_of_two() {
        assert!(!Solution::is_power_of_two(3));
    }
}
