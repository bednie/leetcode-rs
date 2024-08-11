fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn search(i: i32, j: i32, g: &mut Vec<Vec<u8>>) {
            if i >= g.len() as i32 || i < 0 || j >= g[0].len() as i32 || j < 0 {
                return;
            }

            if g[i as usize][j as usize] == 1 {
                g[i as usize][j as usize] = 2;
                for &(di, dj) in &DIRS {
                    search(i + di, j + dj, g);
                }
            }
            return;
        }

        // copy grid into mutable var
        // convert chars to u8 for convenience
        let mut g: Vec<Vec<u8>> = vec![];
        for i in 0..grid.len() {
            g.push(vec![]);
            for j in 0..grid[i].len() {
                g[i].push(grid[i][j] as u8 - b'0' as u8);
            }
        }

        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        let mut count = 0;
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                if g[i][j] == 1 {
                    count += 1;
                    search(i as i32, j as i32, &mut g);
                }
            }
        }
        count
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_num_islands() {
        let v: Vec<Vec<char>> = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(v), 3);
    }
}
