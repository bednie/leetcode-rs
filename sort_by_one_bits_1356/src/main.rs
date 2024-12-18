fn main() {
    dbg!(Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]));
}

struct Solution;

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr.clone();
        arr.sort_by(|a, b| {
            if a.count_ones() == b.count_ones() {
                a.cmp(&b)
            } else {
                a.count_ones().cmp(&b.count_ones())
            }
        });
        arr
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sort_by_bits() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
