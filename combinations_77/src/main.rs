fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    // with help from tmtappr:
    // https://leetcode.com/problems/combinations/solutions/5299120/rust-recursive-iterative
    fn backtrack(
        n: i32,
        k: i32,
        current_combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) -> () {
        if k == 0 {
            result.push(current_combination.to_vec());
            return ();
        }

        let start = current_combination.last().map_or(1, |&n| n + 1);

        for i in start..=n - k + 1 {
            current_combination.push(i);
            Solution::backtrack(n, k - 1, current_combination, result);
            current_combination.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current_combination = vec![];
        Solution::backtrack(n, k, &mut current_combination, &mut result);
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_combine() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                [1, 3].into(),
                [1, 4].into(),
                [2, 3].into(),
                [2, 4].into(),
                [3, 4].into()
            ]
        );
    }
}
