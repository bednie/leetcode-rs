fn main() {
    dbg!(Solution::is_toeplitz_matrix(vec![
        vec![1, 2, 3, 4],
        vec![5, 1, 2, 3],
        vec![9, 5, 1, 2]
    ]));
}

struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for row in 1..matrix.len() {
            for col in 1..matrix[0].len() {
                if matrix[row][col] != matrix[row - 1][col - 1] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_toeplitz_matrix() {
        assert_eq!(
            Solution::is_toeplitz_matrix(vec![
                vec![1, 2, 3, 4],
                vec![5, 1, 2, 3],
                vec![9, 5, 1, 2]
            ]),
            true
        );
    }
}
