fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    // wanting to learn about flatten
    //  ignoring time complexity constraint because i already solved
    //  in O(logm+logn) time in python
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix
            .iter()
            .flatten()
            .collect::<Vec<&i32>>()
            .contains(&&target)
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_search_matrix() {
        assert!(Solution::search_matrix(vec![vec![1], vec![2], vec![3]], 1));
    }
}
