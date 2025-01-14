fn main() {
    dbg!(Solution::kth_smallest(
        vec![[1, 5, 9].to_vec(), [10, 11, 13].into(), [12, 13, 15].into()],
        8
    ));
}

struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut flattened = matrix.into_iter().flatten().collect::<Vec<i32>>();
        flattened.sort_unstable();
        flattened[k as usize - 1]
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        assert_eq!(
            Solution::kth_smallest(
                vec![[1, 5, 9].to_vec(), [10, 11, 13].into(), [12, 13, 15].into()],
                8
            ),
            13
        );
    }
}
