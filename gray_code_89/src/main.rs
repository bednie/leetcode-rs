fn main() {
    let res = Solution::gray_code(4);
    dbg!(res);
}

struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..2usize.pow(n as u32))
            .into_iter()
            .map(|idx| (idx ^ (idx >> 1)) as i32)
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_gray_code() {
        assert_eq!(
            Solution::gray_code(4), 
            vec![0,1,3,2,6,7,5,4,12,13,15,14,10,11,9,8]
        );
    }
}
