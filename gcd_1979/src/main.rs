fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                return a;
            }
            gcd(b, a % b)
        }
        gcd(*nums.iter().max().unwrap(), *nums.iter().min().unwrap())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_gcd() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
    }
}
