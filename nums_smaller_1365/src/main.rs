fn main() {
    dbg!(Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut s = std::collections::HashMap::new();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        for (idx, n) in sorted_nums.into_iter().enumerate() {
            s.entry(n).or_insert(idx);
        }

        nums.into_iter()
            .map(|n| *s.get(&n).unwrap() as i32)
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_smaller_numbers_than_current() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }
}
