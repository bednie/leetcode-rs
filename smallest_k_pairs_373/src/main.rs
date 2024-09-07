// With help from:
// SalvadorDali: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/3701758/rust-python-heap-with-explanation
// williamcs: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/3687915/rust-heap-solution

use std::collections::BinaryHeap;

fn main() {
    dbg!(Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3));
}

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut min_heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let mut result: Vec<Vec<i32>> = vec![];

        // think of the pairs as a table.
        // by first going right (along nums1, filling in the first row), 
        // and then going down or down + right, we don't need to worry 
        // about hitting the same pair of indices more than once.
        (0..nums1.len()).for_each(|x| min_heap.push((-(nums1[x] + nums2[0]), x, 0)));

        while let (Some((_, i, j)), true) = (min_heap.pop(), result.len() < k as usize) {
            result.push(vec![nums1[i], nums2[j]]);
            if j + 1 < nums2.len() {
                min_heap.push((-(nums1[i] + nums2[j + 1]), i, j + 1));
            }
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_k_smallest_pairs() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![[1, 2], [1, 4], [1, 6]]
        );
    }
}
