struct Solution {  }

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        match nums.len() == 1 {
            true => return nums[0],
            false => (),
        }

        let mut start: usize = 0;
        let mut end: usize = nums.len();

        loop {
            if nums[start..end].len() <= 3 {
                    break;
                }
            let mut midpoint = end - (end - start) / 2;
            if nums[midpoint - 1] == nums[midpoint] {
                        midpoint -= 1;
                    }
            if nums[start..midpoint].len() % 2 == 1 {
                        end = midpoint;
                    } else {
                        start = midpoint;
                    }
        }

        match nums[start] == nums[start + 1] {
            true => return nums[end - 1],
            false => return nums[start],
        }
    }
}

fn main() {
    println!("{}",Solution::single_non_duplicate(vec![1,1,2]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_array() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
    }

    #[test]
    fn test_trivial_array() {
        assert_eq!(Solution::single_non_duplicate(vec![1]), 1);
    }

    #[test]
    #[should_panic]
    fn test_empty_array() {
        Solution::single_non_duplicate(vec![]);
    }

    #[test]
    fn test_larger_array() {
        assert_eq!(Solution::single_non_duplicate(vec![1, 1, 3, 3, 7, 7, 10, 11, 11, 12, 12, 13, 13]),10);
    }
}