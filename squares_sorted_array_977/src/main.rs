fn main() {
    dbg!(Solution::sorted_squares(vec![-7, -3, 2, 3, 11]));
}

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut low, mut high) = (0, nums.len());
        let mut result = vec![0; nums.len()];

        while low < high {
            let l = nums[low] * nums[low];
            let h = nums[high - 1] * nums[high - 1];
            if l > h {
                result[high - low - 1] = l;
                low += 1;
            } else {
                result[high - low - 1] = h;
                high -= 1;
            }
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
