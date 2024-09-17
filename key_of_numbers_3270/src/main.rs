fn main() {
    dbg!(Solution::generate_key(987, 879, 798));
}

struct Solution;

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut key = std::cmp::min(num1 % 10, std::cmp::min(num2 % 10, num3 % 10));

        key += std::cmp::min(
            num1 / 10 % 10,
            std::cmp::min(num2 / 10 % 10, num3 / 10 % 10),
        ) * 10;

        key += std::cmp::min(
            num1 / 100 % 10,
            std::cmp::min(num2 / 100 % 10, num3 / 100 % 10),
        ) * 100;

        key += std::cmp::min(
            num1 / 1000 % 10,
            std::cmp::min(num2 / 1000 % 10, num3 / 1000 % 10),
        ) * 1000;

        key
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_generate_key() {
        assert_eq!(Solution::generate_key(987, 879, 798), 777);
    }
}
