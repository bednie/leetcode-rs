fn main() {
    dbg!(Solution::max_width_of_vertical_area(vec![
        vec![3, 1],
        vec![9, 0],
        vec![1, 0],
        vec![1, 4],
        vec![5, 3],
        vec![8, 8]
    ]));
}

struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut p = points;
        p.sort_unstable();
        p.windows(2).map(|w| w[1][0] - w[0][0]).max().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_max_width_of_vertical_area() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3
        );
    }
}
