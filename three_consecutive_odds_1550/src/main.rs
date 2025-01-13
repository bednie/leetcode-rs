fn main() {
    dbg!(Solution::three_consecutive_odds(vec![
        1, 2, 34, 3, 4, 5, 7, 23, 12
    ]));
}

struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.iter().fold(0, |odds, n| {
            if odds > 2 {
                odds
            } else if n % 2 == 1 {
                odds + 1
            } else {
                0
            }
        }) > 2
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_three_consecutive_odds() {
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
    }
}
