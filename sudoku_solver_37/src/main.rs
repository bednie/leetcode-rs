fn main() {
    let mut puzzle: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let answer: Vec<Vec<char>> = vec![
        vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];
    Solution::solve_sudoku(&mut puzzle);
    
    dbg!(puzzle == answer);
}

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn is_valid(num: char, row: usize, col: usize, board: &Vec<Vec<char>>) -> bool {
            let grid_row = (row / 3) * 3;
            let grid_col = (col / 3) * 3;

            for r in grid_row..grid_row + 3 {
                for c in grid_col..grid_col + 3 {
                    if board[r][c] == num {
                        return false;
                    }
                }
            }

            for r in 0..9 {
                if board[r][col] == num {
                    return false;
                }
            }

            for c in 0..9 {
                if board[row][c] == num {
                    return false;
                }
            }

            return true;
        }

        fn backtrack(row: usize, col: usize, board: &mut Vec<Vec<char>>) -> bool {
            if col > 8 {
                return backtrack(row + 1, 0, board);
            }

            if row > 8 {
                return true;
            }

            if board[row][col] != '.' {
                return backtrack(row, col + 1, board);
            }

            for num in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                if is_valid(num, row, col, board) {
                    board[row][col] = num;

                    if backtrack(row, col + 1, board) {
                        return true;
                    }

                    board[row][col] = '.';
                }
            }
            false
        }
        backtrack(0, 0, board);
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_solve_sudoku() {
        let mut puzzle: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let answer: Vec<Vec<char>> = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        Solution::solve_sudoku(&mut puzzle);

        assert_eq!(puzzle, answer);
    }
}
