fn main() {
    let mut v = vec![1, 3, 2];
    Solution::next_permutation(&mut v);
    dbg!(v);
}

struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut k = 101_usize;

        for idx in (0..nums.len() - 1).rev() {
            if nums[idx] < nums[idx + 1] {
                k = idx;
                break;
            }
        }

        if k == 101 {
            nums.reverse();
            return;
        }

        for idx in (k + 1..nums.len()).rev() {
            if nums[k] < nums[idx] {
                nums.swap(k, idx);
                break;
            }
        }

        nums[k + 1..].reverse();
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut test_1 = vec![1, 2, 3];
        let mut test_2 = vec![3, 2, 1];
        let mut test_3 = vec![1, 1, 5];

        Solution::next_permutation(&mut test_1);
        Solution::next_permutation(&mut test_2);
        Solution::next_permutation(&mut test_3);

        assert_eq!(test_1, vec![1, 3, 2]);
        assert_eq!(test_2, vec![1, 2, 3]);
        assert_eq!(test_3, vec![1, 5, 1]);
    }
}
