fn main() {
    dbg!(Solution::check_valid(vec![vec![1,2,3],vec![3,1,2],vec![2,3,1]]));
}

struct Solution;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let length = matrix.len();
        let all: u128 = u128::MAX >> 128 - matrix.len();

        for row in 0..length {
            let mut check: u128 = 0;
            for col in 0..length {
                check |= 1 << matrix[row][col] - 1;
            }
            if check != all {
                return false;
            }
        }

        for col in 0..length {
            let mut check: u128 = 0;
            for row in 0..length {
                check |= 1 << matrix[row][col] - 1;
            }
            if check != all {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_check_valid() {
        assert!(Solution::check_valid(vec![vec![1,2,3],vec![3,1,2],vec![2,3,1]]));
    }
}