fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let n = n as u32;
        let lz = n.leading_zeros().min(31);
        (u32::MAX - n << lz >> lz) as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_complement() {
        assert_eq!(Solution::bitwise_complement(5), 2);
    }
}