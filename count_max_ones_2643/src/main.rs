fn main() {
    dbg!(Solution::row_and_maximum_ones(vec![
        vec![0, 0, 0],
        vec![0, 1, 1]
    ]));
}

struct Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0_i32; 2];

        for (idx, row) in mat.iter().enumerate() {
            let count = row.iter().sum();
            if count > result[1] {
                result[0] = idx as i32;
                result[1] = count;
            }
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_row_and_maximum_ones() {
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
            vec![1, 2]
        );
    }
}
