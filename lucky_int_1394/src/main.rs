fn main() {
    dbg!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]));
}

struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut luck = vec![0_usize; 501];

        for num in arr {
            luck[num as usize] += 1;
        }

        for idx in (1..luck.len()).rev() {
            if luck[idx] == idx {
                return idx as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_lucky() {
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    }
}
