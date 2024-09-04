fn main() {
    dbg!(Solution::single_number(vec![1, 1, 2, 2, 3, 5]));
}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc: i32, &num| acc ^ num);
        let mut ans: Vec<i32> = vec![0; 2];

        let partitioner: i64 = 1 << xor.trailing_zeros();

        nums.into_iter().for_each(|num: i32| {
            if partitioner & num as i64 > 0 {
                ans[0] ^= num
            } else {
                ans[1] ^= num
            }
        });
        ans
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 2, 3, 5]), vec![3, 5]);
    }
}
