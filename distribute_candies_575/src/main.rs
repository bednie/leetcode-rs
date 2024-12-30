fn main() {
    dbg!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]));
}

struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        let candies: std::collections::HashSet<i32> =
            std::collections::HashSet::from_iter(candy_type);
        (n / 2).min(candies.len()) as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    }
}
