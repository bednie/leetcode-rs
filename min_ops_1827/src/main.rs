fn main() {
    dbg!(Solution::min_operations(vec![1, 5, 2, 4, 1]));
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        nums.windows(2)
            .fold((0, 0), |(ops, n), w| {
                let n = 0.max(w[0] - w[1] + n + 1);
                (ops + n, n)
            })
            .0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
    }
}
