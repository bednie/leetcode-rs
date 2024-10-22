fn main() {
    dbg!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2));
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut collection: i32 = (k * (k + 1)) / 2;
        let mut seen: Vec<i32> = vec![0; 51];
        let mut ops: i32 = 0;

        for n in nums.iter().rev() {
            ops += 1;
            if *n <= k && seen[*n as usize] == 0 {
                collection -= n;
                seen[*n as usize] += 1;
            }

            if collection == 0 {
                break;
            }
        }
        ops
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(Solution::min_operations(vec![3, 1, 5, 4, 2], 2), 4);
    }
}
