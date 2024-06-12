fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut ans = 0_u64;
        let mut i = 1_u64 << 32 - x.leading_zeros() as u64;     
        while i != 0 {
            if (ans + i) * (ans + i) <= x as u64 {
                ans += i;
            }
            i >>= 1;
        }
        ans as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(
            Solution::my_sqrt(2147395599), 46339);
    }
}