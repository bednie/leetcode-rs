use std::cmp::{max, min};

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for i in prices.iter() {
            min_price = min(*i, min_price);
            max_profit = max(max_profit, *i - min_price);
        }
        max_profit
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
    }
}