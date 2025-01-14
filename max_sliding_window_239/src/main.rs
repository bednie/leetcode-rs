fn main() {
    dbg!(Solution::max_sliding_window(
        vec![1, 3, -1, -3, 5, 3, 6, 7],
        3
    ));
}

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut m: std::collections::BinaryHeap<(i32, usize)> = std::collections::BinaryHeap::new();
        let mut start = 0_usize;
        let mut result = vec![];

        for (idx, n) in nums[..k as usize].iter().enumerate() {
            m.push((*n, idx));
        }

        result.push(m.peek().unwrap().0);

        while (start + k as usize) < nums.len() {
            m.push((nums[start + k as usize], start + k as usize));

            while m.peek().unwrap().1 <= start {
                m.pop();
            }

            result.push(m.peek().unwrap().0);

            start += 1;
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
