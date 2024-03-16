struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut answer: i32 = 0;
        for i in nums.iter() { 
            answer ^= i;
        }
        answer
    }
}

fn main() {
    println!("{:?}",Solution::single_number(vec![4, 1, 2, 1, 2]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_array() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}