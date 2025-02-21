fn main() {
    dbg!(Solution::tictactoe(vec![
        vec![0, 0],
        vec![2, 0],
        vec![1, 1],
        vec![2, 1],
        vec![2, 2]
    ]));
}

struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        fn check(board: &[Vec<i32>]) -> bool {
            board[0][0] == board[0][1] && board[0][1] == board[0][2]
                || board[1][0] == board[1][1] && board[1][1] == board[1][2]
                || board[2][0] == board[2][1] && board[2][1] == board[2][2]
                || board[0][0] == board[1][0] && board[1][0] == board[2][0]
                || board[0][1] == board[1][1] && board[1][1] == board[2][1]
                || board[0][2] == board[1][2] && board[1][2] == board[2][2]
                || board[0][0] == board[1][1] && board[1][1] == board[2][2]
                || board[0][2] == board[1][1] && board[1][1] == board[2][0]
        }

        let mut board = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];

        for (mv, m) in moves.iter().enumerate() {
            board[m[0] as usize][m[1] as usize] = if mv % 2 == 0 { 65 } else { 66 };

            if mv >= 2 && check(&board) {
                return (board[m[0] as usize][m[1] as usize] as u8 as char).to_string();
            }
        }

        if moves.len() == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_tictactoe() {
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ]),
            "A".to_string()
        );
    }
}
