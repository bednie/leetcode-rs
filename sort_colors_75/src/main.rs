fn main() {
    let mut v: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
    dbg!(Solution::sort_colors(&mut v));
    dbg!(v);
}

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        nums.sort_unstable()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut v: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);
    }
}
