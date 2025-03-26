fn main() {
    dbg!(Solution::find_safe_walk(
        vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0]
        ],
        1
    ));
}

struct Solution;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let mut v = vec![vec![0_i32; grid[0].len()]; grid.len()];
        let mut q = std::collections::VecDeque::<(usize, usize, i32)>::from([(
            0_usize,
            0_usize,
            health - grid[0][0],
        )]);

        while let Some(p) = q.pop_front() {
            let (i, j, h) = p;

            if i == grid.len() - 1 && j == grid[0].len() - 1 && h > 0 {
                return true;
            }

            // up
            if i > 0 && (h - grid[i - 1][j]) > v[i - 1][j] {
                v[i - 1][j] = h - grid[i - 1][j];
                q.push_back((i - 1, j, h - grid[i - 1][j]));
            }

            // down
            if i + 1 < grid.len() && (h - grid[i + 1][j]) > v[i + 1][j] {
                v[i + 1][j] = h - grid[i + 1][j];
                q.push_back((i + 1, j, h - grid[i + 1][j]));
            }

            // right
            if j + 1 < grid[0].len() && (h - grid[i][j + 1]) > v[i][j + 1] {
                v[i][j + 1] = h - grid[i][j + 1];
                q.push_back((i, j + 1, h - grid[i][j + 1]));
            }

            // left
            if j > 0 && (h - grid[i][j - 1]) > v[i][j - 1] {
                v[i][j - 1] = h - grid[i][j - 1];
                q.push_back((i, j - 1, h - grid[i][j - 1]));
            }
        }

        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_safe_walk() {
        assert!(Solution::find_safe_walk(
            vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ],
            1
        ));
        assert!(!Solution::find_safe_walk(
            vec![
                vec![0, 1, 1, 0, 0, 0],
                vec![1, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 1],
                vec![0, 0, 1, 0, 1, 0]
            ],
            3
        ));
    }
}
