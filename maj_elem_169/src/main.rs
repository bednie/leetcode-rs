fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut element: i32 = nums[0];

        for i in 1..nums.len() {
            if nums[i] == element {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    element = nums[i + 1];
                }
            }
        }
        element
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
