use std::iter::zip;

struct Solution {}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {

        // trivial case 
        match heights.len() {
            0 => panic!("Invalid input!"),
            1 => return 0,
            2.. => {},
        }

        let mut expected_heights: Vec<i32> = heights.clone();
        expected_heights.sort_unstable();
        let mut non_matching_indices: i32 = 0;

        for (a, b) in zip(heights.iter(), expected_heights.iter()) {
            match a == b {
                true => {}
                false => non_matching_indices += 1,
            }
        }
        non_matching_indices
    }
}

fn main() {
    println!("{:?}", Solution::height_checker(vec![4, 1, 2, 1, 2]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_array() {
        assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }
}
