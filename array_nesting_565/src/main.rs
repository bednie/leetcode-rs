fn main() {
    dbg!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]));
}

struct Solution;

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut max_length = 0;
        let mut seen = vec![false; nums.len()];

        for num in 0..nums.len() {
            if seen[num] {
                continue;
            }

            let mut h = std::collections::HashSet::new();
            let mut next = num;

            while !h.contains(&next) {
                h.insert(next);
                seen[next] = true;
                next = nums[next] as usize;
            }

            max_length = max_length.max(h.len());
        }
        max_length as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_array_nesting() {
        assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    }
}
