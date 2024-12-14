fn main() {
    let mut v: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut v);
    dbg!(v);
}

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut start = 0;
        let mut end = nums.len() - 1;
        
        for idx in 0..nums.len() {
            if nums[idx] == 0 {
                nums.swap(idx, start);
                start += 1;
            }
        }

        for idx in (0..nums.len()).rev() {
            if idx < start {
                break;
            }

            if nums[idx] == 2 {
                nums.swap(idx, end);
                end -= 1;
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_sort_colors_2() {
        let mut v = vec![3, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_ne!(v, vec![0, 0, 1, 1, 2, 2]);
    }
}
