fn main() {
    dbg!(Solution::missing_rolls(vec![1, 5, 6], 3, 4));
}

struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut current_sum: i32 = rolls.iter().sum();
        let current_mean: i32 = current_sum / rolls.len() as i32;

        if current_mean < mean && ((current_sum + (6 * n)) / (n + rolls.len() as i32)) < mean {
            return vec![];
        }

        if current_mean > mean && ((current_sum + n) / (n + rolls.len() as i32)) > mean {
            return vec![];
        }

        if current_mean == mean {
            return vec![mean; n as usize];
        }

        let guess = ((mean * (rolls.len() as i32 + n)) - current_sum) / n;
        let mut complement = vec![guess; n as usize];
        current_sum += n * guess;

        #[allow(clippy::needless_range_loop)]
        for i in 0..n as usize {
            if current_sum / (n + rolls.len() as i32) == mean {
                return complement;
            }

            let distance = mean - (current_sum / (n + rolls.len() as i32));
            current_sum += distance;
            complement[i] += distance;
        }
        vec![]
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
