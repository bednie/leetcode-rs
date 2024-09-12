fn main() {
    dbg!(Solution::missing_rolls(vec![1, 5, 6], 3, 4));
}

struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let missing_sum = mean * (n + rolls.len() as i32) - rolls.iter().sum::<i32>();

        if missing_sum < n || missing_sum > 6 * n { 
            return vec![]; 
        }

        let mut result = vec![missing_sum/n; n as usize];
        (0..missing_sum % n).for_each(|i| result[i as usize] += 1);
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_missing_rolls() {
        assert_eq!(
            Solution::missing_rolls(vec![1, 5, 6], 3, 4),
            vec![3, 2, 2, 2]
        );

        assert_eq!(Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
        assert_eq!(Solution::missing_rolls(vec![1, 2, 4, 3], 6, 4), vec![]);
    }
}
