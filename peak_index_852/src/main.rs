fn main() {
    dbg!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]));
}

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let arr_idx: Vec<(usize, i32)> = arr
            .into_iter()
            .enumerate()
            .collect();
        let mut left = 0;
        let mut right = arr_idx.len();

        while left < right {
            let mid = (left + right) / 2;

            if arr_idx[mid].1 > arr_idx[mid - 1].1 && arr_idx[mid].1 > arr_idx[mid + 1].1 {
                return arr_idx[mid].0 as i32;
            } else if arr_idx[mid].1 > arr_idx[mid - 1].1 && arr_idx[mid].1 < arr_idx[mid + 1].1 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_peak_index_in_mountain_array() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    }
}
