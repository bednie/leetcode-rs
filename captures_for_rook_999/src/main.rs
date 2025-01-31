fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
    ];

    dbg!(Solution::num_rook_captures(board));
}

struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut rook = (0, 0);
        let mut pawns = 0;

        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    rook = (i as usize, j as usize);
                    break;
                }
            }
        }

        // up
        for i in (0..rook.0).rev() {
            if board[i][rook.1] == 'p' {
                pawns += 1;
                break;
            } else if board[i][rook.1] == '.' {
                continue;
            } else {
                break;
            }
        }

        // down
        for i in rook.0 + 1..8 {
            if board[i][rook.1] == 'p' {
                pawns += 1;
                break;
            } else if board[i][rook.1] == '.' {
                continue;
            } else {
                break;
            }
        }

        // left
        for j in (0..rook.1).rev() {
            if board[rook.0][j] == 'p' {
                pawns += 1;
                break;
            } else if board[rook.0][j] == '.' {
                continue;
            } else {
                break;
            }
        }

        // right
        for j in rook.1 + 1..8 {
            if board[rook.0][j] == 'p' {
                pawns += 1;
                break;
            } else if board[rook.0][j] == '.' {
                continue;
            } else {
                break;
            }
        }
        pawns
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_num_rook_captures() {
        let board: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board), 3);
    }
}
