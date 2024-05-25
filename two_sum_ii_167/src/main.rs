fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;

        while i < j {
            if numbers[i] + numbers[j] == target {
                return vec![i as i32 + 1, j as i32 + 1];
            }

            if numbers[i] + numbers[j] > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
        vec![i as i32 + 1, j as i32 + 1]
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
