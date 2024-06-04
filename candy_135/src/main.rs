fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 1 {
            return 1;
        }
        let mut candies = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1
            }
        }

        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1] {
                candies[i] = candies[i + 1] + 1
            }
        }
        candies.into_iter().sum::<i32>()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_candy() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }
}
