fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        let mut length = nums.len();

        while fast < length {
            while fast < length && nums[slow] == nums[fast] && fast - slow > 1 {
                nums.remove(fast);
                length -= 1;
            }

            if fast < length && nums[slow] != nums[fast] {
                slow = fast;
            }

            fast += 1;
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            Solution::remove_duplicates(&mut Vec::from([1, 1, 1, 2, 2, 3])),
            5
        );
    }
}
