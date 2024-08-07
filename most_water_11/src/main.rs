use std::cmp::min;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut max_area, mut i, mut j) = (0, 0, height.len() - 1);
        while i <= j {
            let area = min(height[i], height[j]) * (j - i) as i32;
            if area > max_area {
                max_area = area;
            }

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
