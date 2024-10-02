fn main() {
    dbg!(Solution::get_row(3));
}

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut triangle: Vec<Vec<i32>> = vec![vec![1]];

        for row in 1..=row_index as usize {
            let mut new_row = vec![1; row + 1];

            for r in 1..row {
                new_row[r] = triangle[row - 1][r - 1] + triangle[row - 1][r];
            }

            triangle.push(new_row);
        }
        triangle.pop().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_get_row() {
        assert_eq!(
            Solution::get_row(33),
            vec![
                1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040,
                193536720, 354817320, 573166440, 818809200, 1037158320, 1166803110, 1166803110,
                1037158320, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100,
                13884156, 4272048, 1107568, 237336, 40920, 5456, 528, 33, 1
            ]
        );
    }
}
