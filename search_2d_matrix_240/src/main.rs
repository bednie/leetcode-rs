fn main() {
    dbg!(Solution::search_matrix(
        vec![
            [1, 4, 7, 11, 15].to_vec(),
            [2, 5, 8, 12, 19].into(),
            [3, 6, 9, 16, 22].into(),
            [10, 13, 14, 17, 24].into(),
            [18, 21, 23, 26, 30].into()
        ],
        5
    ));
}

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix.iter().any(|row| {
            if target <= *row.last().unwrap() && target >= row[0] {
                row.binary_search(&target).is_ok()
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert!(
            Solution::search_matrix(
                vec![
                    [1, 4, 7, 11, 15].into(),
                    [2, 5, 8, 12, 19].into(),
                    [3, 6, 9, 16, 22].into(),
                    [10, 13, 14, 17, 24].into(),
                    [18, 21, 23, 26, 30].into()
                ],
                5
            ) && !Solution::search_matrix(
                vec![
                    [1, 4, 7, 11, 15].into(),
                    [2, 5, 8, 12, 19].into(),
                    [3, 6, 9, 16, 22].into(),
                    [10, 13, 14, 17, 24].into(),
                    [18, 21, 23, 26, 30].into()
                ],
                20
            )
        );
    }
}
