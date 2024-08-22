fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        ((i32::MAX >> 32 - num.leading_zeros()) << 32 - num.leading_zeros()) ^ (i32::MAX - num)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_complement() {
        assert_eq!(Solution::find_complement(5), 2);
    }
}
