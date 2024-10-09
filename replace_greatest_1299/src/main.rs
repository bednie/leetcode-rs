fn main() {
    dbg!(Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]));
}

struct Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr.clone();
        let length = arr.len();
        let mut max_element = arr[length - 1];
        arr[length - 1] = -1;

        (0..arr.len() - 1).rev().for_each(|idx| {
            let temp = arr[idx];
            arr[idx] = max_element;
            max_element = max_element.max(temp)
        });
        arr
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_replace_elements() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            [18, 6, 6, 6, 1, -1]
        );
    }
}
