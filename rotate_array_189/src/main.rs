use std::{borrow::Borrow, mem};

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) -> &Vec<i32> {
        let mut k = k % nums.len() as i32;
        while k > 0 {
            let ele = nums.pop().unwrap();
            nums.insert(0, ele);
            k -= 1;
        }
        nums
    }
}

struct Solution2;

impl Solution2 {
    pub fn rotate(mut nums: &mut Vec<i32>, k: i32) -> &Vec<i32> {
        let i: usize = nums.len() - (k as usize % nums.len());
        let x = nums.drain(0..i).as_slice().to_owned();
        nums.extend_from_slice(&x);
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(
            Solution::rotate(&mut vec![1, 2, 3, 4, 5, 6, 7], 3),
            &vec![5, 6, 7, 1, 2, 3, 4]
        )
    }

    #[test]
    fn test_rotate_2() {
        assert_eq!(
            Solution2::rotate(&mut vec![1, 2, 3, 4, 5, 6, 7], 3),
            &vec![5, 6, 7, 1, 2, 3, 4]
        )
    }
}
