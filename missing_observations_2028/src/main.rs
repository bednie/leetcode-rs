fn main() {
    dbg!(Solution::missing_rolls(vec![1, 5, 6], 3, 4));
}

struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let mut missing = (mean * (n + m)) - rolls.iter().sum::<i32>();

        //println!("{:#?}", missing);

        if missing < n || missing > 6 * n { 
            return vec![]; 
        }

        let roll = missing / n;
        missing -= roll * n;
        let mut result = vec![roll; n as usize];
        result[0] += missing;
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
