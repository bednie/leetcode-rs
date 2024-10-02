fn main() {
    dbg!(Solution::generate(3));
}

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = vec![vec![1]];

        for row in 1..num_rows as usize {
            let mut new_row = vec![1; row + 1];

            for r in 1..row {
                new_row[r] = triangle[row - 1][r - 1] + triangle[row - 1][r];
            }

            triangle.push(new_row);
        }
        triangle
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_get_row() {
        assert_eq!(
            Solution::generate(3),
            vec![vec![1], vec![1, 1], vec![1, 2, 1]]
        );
    }
}
