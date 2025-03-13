fn main() {
    dbg!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
}

struct Solution;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        fn next_idx(idx: usize, nums: &[i32]) -> usize {
            let n = (idx as i32 + nums[idx]) % nums.len() as i32;
            if n < 0 {
                (nums.len() as i32 + n) as usize
            } else {
                n as usize
            }
        }

        for idx in (0..nums.len()).rev() {
            let mut v = vec![false; nums.len()];
            let mut current = idx;

            while !v[current] {
                v[current] = true;

                let next = next_idx(current, &nums);

                if current == next || (nums[next] > 0) != (nums[current] > 0) {
                    v[next] = true;
                    break;
                }

                if v[next] {
                    return true;
                }

                current = next;
            }
        }

        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_circular_array_loop() {
        assert!(!Solution::circular_array_loop(vec![-1, -2, -3, -4, -5, 6]));
        assert!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
    }
}
