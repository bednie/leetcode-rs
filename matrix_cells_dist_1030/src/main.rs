fn main() {
    dbg!(Solution::all_cells_dist_order(2, 3, 1, 2));
}

struct Solution;

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut m = vec![];

        for r in 0..rows {
            for c in 0..cols {
                m.push(vec![r, c]);
            }
        }

        m.sort_unstable_by_key(|cell| (r_center - cell[0]).abs() + (c_center - cell[1]).abs());
        m
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_all_cells_dist_order() {
        assert_eq!(
            Solution::all_cells_dist_order(2, 3, 1, 2),
            vec![
                vec![1, 2],
                [0, 2].into(),
                [1, 1].into(),
                [0, 1].into(),
                [1, 0].into(),
                [0, 0].into()
            ]
        );
    }
}
