fn main() {
    println!("{:32b}",i32::MAX);
}

struct Solution;

impl Solution {
    // i used this solution from soooch:
    // https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/4761970/rust-beats-100-0-ms-o-1-optimal-with-explanation
    // https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/2073976/compiles-to-five-instructions-on-x86-no-loops-no-branches
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let xor = right ^ left;
        let clear_n_bits = 32 - xor.leading_zeros();
        (right >> clear_n_bits) << clear_n_bits 
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }
}
