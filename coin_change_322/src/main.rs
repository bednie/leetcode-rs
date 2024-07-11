fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut memo = vec![i32::MAX; amount + 1];
        memo[0] = 0;

        for coin in coins.iter() {
            let coin = *coin as usize;
            for i in coin..=amount {
                if memo[i - coin] != i32::MAX {
                    memo[i] = memo[i].min(memo[i - coin] + 1);
                }
            }
        }

        if memo[amount as usize] == i32::MAX {
            return -1;
        }
        memo[amount]
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }
}
