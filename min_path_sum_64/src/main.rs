fn main() {
    dbg!(Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]));
}

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid: Vec<Vec<i32>> = grid;

        for i in 1..grid.len() {
            grid[i][0] += grid[i - 1][0];
        }

        for i in 1..grid[0].len() {
            grid[0][i] += grid[0][i - 1];
        }

        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                grid[i][j] += grid[i][j - 1].min(grid[i - 1][j]);
            }
        }
        grid[grid.len() - 1][grid[0].len() - 1]
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );

        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }
}
