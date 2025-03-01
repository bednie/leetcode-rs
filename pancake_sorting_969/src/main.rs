fn main() {
    dbg!(Solution::pancake_sort(vec![3, 2, 4, 1]));
}

struct Solution;

impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut max_elem = arr.len() as i32;
        let mut result = vec![];

        while max_elem > 1 {
            let max_idx = arr[..max_elem as usize]
                .iter()
                .position(|&m| m == max_elem)
                .unwrap();

            if max_idx as i32 + 1 != max_elem {
                arr[..=max_idx].reverse();
                arr[..max_elem as usize].reverse();

                result.push(max_idx as i32 + 1);
                result.push(max_elem);
            }

            max_elem -= 1;
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_pancake_sort() {
        assert_eq!(
            Solution::pancake_sort(vec![3, 2, 4, 1]),
            vec![3, 4, 2, 3, 1, 2]
        );
    }
}
