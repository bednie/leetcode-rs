fn main() {
    dbg!(Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
}

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        nums.windows(k as usize).fold(f64::MIN, |m, w| {
            m.max(w.iter().sum::<i32>() as f64 / k as f64)
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_max_average() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75000_f64
        );
    }
}
