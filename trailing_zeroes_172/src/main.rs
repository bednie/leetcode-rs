fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        (n / 5 + n / 25 + n / 125 + n / 625 + n / 3125) as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(Solution::trailing_zeroes(3125), 781);
    }
}
