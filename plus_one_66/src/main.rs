fn main() {
    println!("Hello, world!");
}

struct Solution;

// reimplement underdogo's solution now that digits is not mut
// https://leetcode.com/problems/plus-one/solutions/4791921/rust-1ms-handle-all-9s
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut d = digits.clone();
        for i in (0..d.len()).rev() {
            if d[i] < 9 {
                d[i] += 1;
                return d;
            }
            d[i] = 0;
        }
        
        let mut v = vec![0; digits.len() + 1];
        v[0] = 1;
        v
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }
    
    #[test]
    fn test_plus_one_2() {
        assert_eq!(Solution::plus_one(vec![9,9]), vec![1,0,0]);
    }
}
