fn main() {
    dbg!(Solution::decompress_rl_elist(vec![1, 2, 3, 4]));
}

struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        nums.chunks(2)
            .for_each(|w| result.append(&mut vec![w[1]; w[0] as usize]));
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_decompress_rl_elist() {
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            [2, 4, 4, 4]
        );
    }
}
