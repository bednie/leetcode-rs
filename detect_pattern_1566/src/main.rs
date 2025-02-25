fn main() {
    dbg!(Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3));
}

struct Solution;

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        for window in arr.windows((m * k) as usize) {
            let pattern = &window[..m as usize];

            if window.chunks(m as usize).all(|chunk| chunk == pattern) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_contains_pattern() {
        assert!(Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3));
    }
}
