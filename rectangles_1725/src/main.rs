fn main() {
    dbg!(Solution::count_good_rectangles(vec![
        vec![5, 8],
        vec![3, 9],
        vec![5, 12],
        vec![16, 5]
    ]));
}

struct Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut m = std::collections::HashMap::new();

        for r in rectangles {
            m.entry(r[0].min(r[1])).and_modify(|c| *c += 1).or_insert(1);
        }

        *m.get(m.keys().max().unwrap()).unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_count_good_rectangles() {
        assert_eq!(
            Solution::count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]),
            3
        );
    }
}
