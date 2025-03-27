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
                        let (i, j) = p;

                        for d in [1, 0, -1, 0, 1].windows(2) {
                            if (i + d[0] >= 0 && i + d[0] < board.len() as i32)
                                && (j + d[1] >= 0 && j + d[1] < board[0].len() as i32)
                                && board[(i + d[0]) as usize][(j + d[1]) as usize] == 'X'
                            {
                                board[(i + d[0]) as usize][(j + d[1]) as usize] = '.';
                                q.push((i + d[0], j + d[1]));
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
