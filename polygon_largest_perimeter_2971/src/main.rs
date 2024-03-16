struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        match nums.len() {
            0 => panic!("Empty array!"),
            1 | 2 => -1,
            _ => Solution::test_polygon(nums),
        }
    }

    pub fn test_polygon(nums: Vec<i32>) -> i64 {
        // sort input
        let mut nums_sorted = nums;
        nums_sorted.sort_unstable();

        // sum and map to i64
        let sum: i64 = nums_sorted.iter().map(|&i| i as i64).sum();
        
        // set up initial values
        let mut last_index = nums_sorted.len() - 1;
        let mut last_index_value = nums_sorted[last_index] as i64;
        let mut current_sum = sum;

        while last_index >= 2 {
            match (current_sum - last_index_value) > last_index_value {
                true => return current_sum,
                false => {
                    current_sum -= last_index_value;
                    last_index -= 1;
                    last_index_value = nums_sorted[last_index] as i64;
                }
            }
        }
        -1
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3])
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_array() {
        assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
    }

    #[test]
    #[should_panic]
    fn test_invalid_array() {
        Solution::largest_perimeter(vec![]);
    }

    #[test]
    fn test_no_polygon_array() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
    }

    #[test]
    fn test_large_array() {
        assert_eq!(
            Solution::largest_perimeter(vec![
                300005055, 352368231, 311935527, 315829776, 327065463, 388851949, 319541150,
                397875604, 311309167, 391897750, 366860048, 359976490, 325522439, 390648914,
                359891976, 369105322, 350430086, 398592583, 354559219, 372400239, 344759294,
                379931363, 308829137, 335032174, 336962933, 380797651, 378305476, 336617902,
                393487098, 301391791, 394314232, 387440261, 316040738, 388074503, 396614889,
                331609633, 374723367, 380418460, 349845809, 318514711, 308782485, 308291996,
                375362898, 397542455, 397628325, 392446446, 368662132, 378781533, 372327607,
                378737987
            ]),
            17876942274
        );
    }
}