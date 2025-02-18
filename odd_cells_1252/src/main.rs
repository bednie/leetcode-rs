fn main() {
    dbg!();
}

struct Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut odds = 0;
        let mut matrix = vec![vec![0; n as usize]; m as usize];

        for i in indices {
            let (r, c) = (i[0] as usize, i[1] as usize);

            for idx in 0..n as usize {
                if matrix[r][idx] % 2 == 0 {
                    odds += 1;
                } else {
                    odds -= 1;
                }

                matrix[r][idx] += 1;
            }

            for idx in 0..m as usize {
                if matrix[idx][c] % 2 == 0 {
                    odds += 1;
                } else {
                    odds -= 1;
                }

                matrix[idx][c] += 1;
            }
        }

        odds
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_odd_cells() {
        assert_eq!(
            Solution::odd_cells(2, 3, vec![vec![0,1],vec![1,1]]),
            6
        );
    }
}
