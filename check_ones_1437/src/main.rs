fn main() {
    dbg!(Solution::k_length_apart(vec![1,0,0,0,1,0,0,1], 2));
}

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return true; 
        }

        let mut spaces = if nums[0] == 0 { k } else { 0 };

        for n in nums.into_iter().skip(1) {
            if n == 0 {
                spaces += 1;
                continue;
            }
            
            if spaces < k {
                return false;
            }
            spaces = 0;
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_k_length_apart() {
        assert!(
            Solution::k_length_apart(vec![1,0,0,0,1,0,0,1], 2)
        );
    }
}