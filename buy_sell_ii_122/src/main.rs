fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i-1] {
                profit += prices[i] - prices[i-1];
            }
        }
        profit
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 7);
    }
}