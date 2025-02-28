fn main() {
    dbg!(Solution::max_product(vec![2, 3, -2, 4]));
}

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((i32::MIN, 1, 1), |(acc, mx, mn), n| {
                let candidate = mx * n;
                let max_p = n.max(candidate).max(n * mn);
                let min_p = candidate.min(n).min(n * mn);
                (acc.max(max_p), max_p, min_p)
            })
            .0
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    }
}
