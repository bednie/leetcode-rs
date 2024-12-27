fn main() {
    dbg!(Solution::majority_element(vec![3,2,3]));
}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

        nums.iter().for_each(|n| {
            *map.entry(*n).or_default() += 1;
        });

        map.into_iter()
            .filter(|&(_, v)| v > (nums.len() / 3))
            .map(|(k, _)| k)
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(
            Solution::majority_element(vec![3,2,3]),
            vec![3]
        );
    }
}
