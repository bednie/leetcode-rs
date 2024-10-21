fn main() {
    dbg!(Solution::unique_paths_with_obstacles(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0]
    ]));
}

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1] == 1 {
            return 0;
        }

        let mut path_grid = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];

        path_grid[0][0] = if obstacle_grid[0][0] == 1 { 0 } else { 1 };

        for i in 1..path_grid[0].len() {
            if obstacle_grid[0][i] == 1 {
                break;
            } else {
                path_grid[0][i] = path_grid[0][i - 1];
            }
        }

        for i in 1..path_grid.len() {
            if obstacle_grid[i][0] == 1 {
                break;
            } else {
                path_grid[i][0] = path_grid[i - 1][0];
            }
        }

        for row in 1..path_grid.len() {
            for col in 1..path_grid[0].len() {
                let above = if obstacle_grid[row - 1][col] == 1 {
                    0
                } else {
                    path_grid[row - 1][col]
                };
                let left = if obstacle_grid[row][col - 1] == 1 {
                    0
                } else {
                    path_grid[row][col - 1]
                };
                path_grid[row][col] = above + left;
            }
        }
        *path_grid.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
    }
}
