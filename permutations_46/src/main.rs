fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    fn backtrack(
        current_permutation: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        nums: &Vec<i32>,
        options: Vec<i32>,
    ) -> () {
        if options.len() == 0 {
            result.push(current_permutation.to_vec());
            return ();
        }

        for (i, j) in options.iter().enumerate() {
            current_permutation.push(*j);
            Solution::backtrack(
                current_permutation,
                result,
                &nums,
                [&options[..i], &options[i + 1..]].concat(),
            );
            current_permutation.pop();
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current_permutation = vec![];
        Solution::backtrack(&mut current_permutation, &mut result, &nums, nums.clone());
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
