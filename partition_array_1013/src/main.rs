fn main() {
    dbg!(Solution::can_three_parts_equal_sum(vec![
        0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
    ]));
}

struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum = arr.iter().sum::<i32>();

        if sum % 3 != 0 {
            return false;
        }

        let partition_sum = sum / 3;
        let mut segments = 0;
        let mut p = 0;
        let mut current_sum = arr[p];

        while p < arr.len() - 1 {
            if current_sum != partition_sum {
                p += 1;
                current_sum += arr[p];
            } else {
                segments += 1;
                p += 1;
                current_sum = arr[p];
            }

            if segments == 2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_can_three_parts_equal_sum() {
        assert!(Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
        ]));
        assert!(!Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1
        ]));
    }
}
