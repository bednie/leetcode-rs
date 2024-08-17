fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        fn unravel(board: &Vec<Vec<i32>>) -> Vec<i32> {
            let mut b = vec![0];

            for (i, row) in board.iter().rev().enumerate() {
                if i % 2 == 1 {
                    b.extend(row.clone().iter().rev());
                } else {
                    b.extend(row.clone().iter());
                }
            }
            b
        }

        let board = unravel(&board);
        let mut deque = std::collections::VecDeque::from([(1usize, 0usize)]); // (pos, moves)
        let mut visited = std::collections::HashSet::from([0usize]); // pos

        while !deque.is_empty() {
            let (pos, moves) = deque.pop_front().unwrap();
            for i in 1..=6 {
                let mut next_pos = pos + i;

                if next_pos > board.len() - 1 {
                    break;
                }

                if board[next_pos] != -1 {
                    next_pos = board[next_pos] as usize;
                }

                if next_pos == board.len() - 1 {
                    return moves as i32 + 1;
                }

                if !visited.contains(&next_pos) {
                    visited.insert(next_pos);
                    deque.push_back((next_pos, moves + 1));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_snakes_and_ladders() {
        let board: Vec<Vec<i32>> = Vec::from(vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ]);
        assert_eq!(Solution::snakes_and_ladders(board), 4);
    }
}
