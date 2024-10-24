fn main() {
    dbg!(Solution::find_the_prefix_common_array(
        vec![1, 3, 2, 4],
        vec![3, 1, 2, 4]
    ));
}

struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_nums: u64 = 0;
        let mut b_nums: u64 = 0;
        let mut result: Vec<i32> = vec![0; a.len()];

        for idx in 0..a.len() {
            a_nums |= 1 << a[idx];
            b_nums |= 1 << b[idx];
            result[idx] = (a_nums & b_nums).count_ones() as i32;
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_the_prefix_common_array() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        );
    }
}
