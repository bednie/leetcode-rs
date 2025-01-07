fn main() {
    dbg!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
}

struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut p = 0;
        let mut s = std::collections::HashSet::new();

        for n in nums {
            let m: i32 = (n + p) % k;

            if !s.contains(&m) {
                s.insert(p);
                p = m;
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
