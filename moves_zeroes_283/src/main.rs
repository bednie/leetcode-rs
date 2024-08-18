fn main() {
    dbg!();
}

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut i, mut j) = (0, 0);
        while i < nums.len() - 1 {
            if nums[i] == 0 {
                j = i + 1;
                while j < nums.len() - 1 && nums[j] == 0 {
                    j += 1
                }
                (nums[i], nums[j]) = (nums[j], nums[i]);
            }
            i += 1;
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_move_zeroes() {
        let mut v = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1,3,12,0,0]);
    }
}
