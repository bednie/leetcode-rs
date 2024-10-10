fn main() {
    dbg!(Solution::trim_mean(vec![
        1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
    ]));
}

struct Solution;

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let five_pct: usize = arr.len() / 20;
        let mut arr_sorted: Vec<i32> = arr.clone();
        arr_sorted.sort_unstable();
        arr_sorted[five_pct..arr.len() - five_pct]
            .iter()
            .map(|x| *x as f64)
            .sum::<f64>()
            / (arr.len() - 2 * five_pct) as f64
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_trim_mean() {
        assert_eq!(
            Solution::trim_mean(vec![
                1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
            ]),
            2.0_f64
        )
    }
}
