/*Intuition
This problem has some annoying edge cases. Using Rust's iterators, we can avoid mutating or copying the input while still getting the simplicity of modifying the input to avoid the edge cases.

Approach
By adding a zero at the beginning and end of the flowerbed, we can treat the whole flowerbed the same way: if we have a series of zeros z, we can place n flowers where n = (z - 1) / 2.

The 1 at the end of our constructed iter is simply to force the else block to catch any trailing zeros.

Complexity
Time complexity:
O(n)

Space complexity:
O(1)
*/

fn main() {
    dbg!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
}

struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let iter = std::iter::once(&0).chain(flowerbed.iter().chain(&[0, 1]));
        let mut places = 0;
        let mut zeros = 0;

        for &plot in iter {
            if plot == 0 {
                zeros += 1;
            } else {
                places += (zeros - 1) / 2; 
                zeros = 0;
            }
        }

        places >= n
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
