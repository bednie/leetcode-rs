fn main() {
    dbg!(Solution::generate_matrix(3));
}

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut m = vec![vec![0; n as usize]; n as usize];
        let mut p = (0, 0);
        let mut c = 1;

        while c < n * n {
            // right
            while p.1 < m[0].len() - 1 && m[p.0][p.1 + 1] == 0 {
                m[p.0][p.1] = c;
                p.1 += 1;
                c += 1;
            }

            // down
            while p.0 < m.len() - 1 && m[p.0 + 1][p.1] == 0 {
                m[p.0][p.1] = c;
                p.0 += 1;
                c += 1;
            }

            // left
            while p.1 > 0 && m[p.0][p.1 - 1] == 0 {
                m[p.0][p.1] = c;
                p.1 -= 1;
                c += 1;
            }

            // up
            while p.0 > 0 && m[p.0 - 1][p.1] == 0 {
                m[p.0][p.1] = c;
                p.0 -= 1;
                c += 1;
            }
        }

        m[p.0][p.1] = c;

        m
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_generate_matrix() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}
