use std::collections::BinaryHeap;

fn main() {
    dbg!(Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3));
}

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut min_heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let mut result: Vec<Vec<i32>> = vec![];

        (0..nums1.len()).for_each(|x| min_heap.push((-(nums1[x] + nums2[0]), x, 0)));

        for i in 0..k as usize {
            if let Some((_, i, j)) = min_heap.pop() {
                result.push(vec![nums1[i], nums2[j]]);

                if j + 1 < nums2.len() {
                    min_heap.push((-(nums1[i] + nums2[j + 1]), i, j + 1));
                }
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
