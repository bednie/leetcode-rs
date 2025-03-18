fn main() {
    dbg!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]));
}

struct Solution;

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let mut prefix: i32 = 0;
        let mut suffix: i32 = nums.iter().sum();

        for (idx, middle) in nums.into_iter().enumerate() {
            if prefix == suffix - middle {
                return idx as i32;
            }

            prefix += middle;
            suffix -= middle;
        }

        -1
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_middle_index() {
        assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
    }
}
