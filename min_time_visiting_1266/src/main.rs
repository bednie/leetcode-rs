fn main() {
    dbg!(Solution::min_time_to_visit_all_points(vec![
        vec![1, 1],
        [3, 4].into(),
        [-1, 0].into()
    ]));
}

struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points.windows(2).fold(0, |acc: i32, w: &[Vec<i32>]| {
            acc + ((w[1][0] - w[0][0]).abs()).max((w[1][1] - w[0][1]).abs())
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_time_to_visit_all_points() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], [3, 4].into(), [-1, 0].into()]),
            7
        );
    }
}
