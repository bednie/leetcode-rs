fn main() {
    dbg!(Solution::count_battleships(vec![
        vec!['X', '.', '.', 'X'],
        vec!['.', '.', '.', 'X'],
        vec!['.', '.', '.', 'X']
    ]));
}

struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut board = board;
        let mut q = vec![];
        let mut count = 0;

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if board[row][col] == 'X' {
                    count += 1;
                    board[row][col] = '.';
                    q.push((row as i32, col as i32));

                    while let Some(p) = q.pop() {
                        for d in [1, 0, -1, 0, 1].windows(2) {
                            let (i, j) = (p.0 + d[0], p.1 + d[1]);
                            
                            if (i >= 0 && i < board.len() as i32)
                                && (j >= 0 && j < board[0].len() as i32)
                                && board[i as usize][j as usize] == 'X'
                            {
                                board[i as usize][j as usize] = '.';
                                q.push((i, j));
                            }
                        }
                    }
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
    fn count_battleships() {
        assert_eq!(
            Solution::count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X']
            ]),
            2
        );
    }
}
