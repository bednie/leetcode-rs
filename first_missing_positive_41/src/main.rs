fn main() {
    dbg!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
}

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut positives = vec![0; nums.len() + 2];

        for n in nums {
            if n >= 0 && n < positives.len() as i32 {
                positives[n as usize] = 1;
            }
        }

        for (idx, n) in positives.into_iter().enumerate().skip(1) {
            if n == 0 {
                return idx as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
