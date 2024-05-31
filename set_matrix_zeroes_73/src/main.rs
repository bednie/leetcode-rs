fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let temp = matrix.clone();

        for (i, j) in temp.iter().enumerate() {
            for k in 0..j.len() {
                if j[k] == 0 {
                    matrix[i].fill_with(Default::default);
                    for l in 0..temp.len() {
                        matrix[l][k] = 0;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_set_zeroes() {
        let mut ans = Vec::from([vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
        Solution::set_zeroes(&mut ans);
        assert_eq!(ans, Vec::from([vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]));
    }
}
