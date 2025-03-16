fn main() {
    dbg!(Solution::maximal_square(vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0']
    ]));
}

struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut prev_row = vec![0_u16; matrix[0].len() + 1];
        let mut current_row = vec![0_u16; matrix[0].len() + 1];
        let mut max_square = 0_u16;

        for r in 1..=matrix.len() {
            for c in 1..=matrix[0].len() {
                if matrix[r - 1][c - 1] == '1' {
                    current_row[c] = 1 + prev_row[c - 1].min(current_row[c - 1]).min(prev_row[c]);
                    max_square = max_square.max(current_row[c]);
                }
            }

            std::mem::swap(&mut current_row, &mut prev_row);
            current_row.fill(0);
        }

        (max_square as i32).pow(2)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_maximal_square() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
    }
}
