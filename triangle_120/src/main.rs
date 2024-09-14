fn main() {
    dbg!(Solution::minimum_total(vec![
        vec![2],
        vec![3, 4],
        vec![6, 5, 7],
        vec![4, 1, 8, 3]
    ]));
}

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        for row in 1..triangle.len() {
            triangle[row][0] += triangle[row - 1][0];

            for i in 1..triangle[row].len() {
                triangle[row][i] += triangle[row - 1][(i - 1).min(triangle[row - 1].len() - 1)]
                    .min(triangle[row - 1][i.min(triangle[row - 1].len() - 1)]);
            }
        }
        *triangle[triangle.len() - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_minimum_total() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
