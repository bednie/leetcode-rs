fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut jumps, mut current, mut next) = (0, 0, 0);

        for i in 0..nums.len()-1 {
            next = std::cmp::max(next, i + nums[i] as usize);

            if i == current {
                jumps += 1;
                current = next;
        
                if current >= nums.len()-1 {
                    break
                }
            }
        }
        jumps
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_jump() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
    }
}