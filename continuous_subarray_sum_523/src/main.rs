fn main() {
    dbg!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
}

struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut v = vec![0];
        let mut s = std::collections::HashSet::new();

        for (idx, n) in nums.iter().enumerate() {
            let m = (n + v[idx]) % k;
            v.push(m);
            if !s.contains(&m) {
                s.insert(v[idx]);
            } else {
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
    fn test_check_subarray_sum() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }
}
