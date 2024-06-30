fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in matrix.iter_mut() {
            i.reverse();
        }

        let mut start = matrix.len() - 1;
        let mut end = 0;
        while start > 0 {
            for i in 0..=start {
                let temp = matrix[start - i][end];
                matrix[start - i][end] = matrix[start][end + i];
                matrix[start][end + i] = temp;
            }
            start -= 1;
            end += 1;
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut v = vec![vec![1, 2, 3], [4, 5, 6].into(), [7, 8, 9].into()];
        Solution::rotate(&mut v);
        assert_eq!(v, vec![vec![7, 4, 1], [8, 5, 2].into(), [9, 6, 3].into()]);
    }
}
