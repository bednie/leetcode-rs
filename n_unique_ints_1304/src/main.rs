fn main() {
    dbg!(Solution::sum_zero(5));
}

struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = if n % 2 == 0 { vec![] } else { vec![0] };
        (1..=n / 2).for_each(|n| {
            result.push(n);
            result.push(n * -1);
        });
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sum_zero() {
        assert_eq!(Solution::sum_zero(5), vec![0, 1, -1, 2, -2]);
    }
}
