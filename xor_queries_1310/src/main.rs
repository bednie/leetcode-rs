fn main() {
    dbg!(Solution::xor_queries(
        vec![1, 3, 4, 8],
        vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
    ));
}

struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries
            .iter()
            .map(|q| -> i32 {
                (q[0] as usize..=q[1] as usize)
                .fold(0, |acc: i32, c: usize| acc ^ arr[c])
            })
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_xor_queries() {
        assert_eq!(
            Solution::xor_queries(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
            ),
            vec![2, 7, 14, 8]
        );
    }
}
