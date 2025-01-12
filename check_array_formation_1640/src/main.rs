fn main() {
    dbg!(Solution::can_form_array(
        vec![15, 88],
        vec![[88].to_vec(), [15].to_vec()]
    ));
}

struct Solution;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut pieces = pieces;
        let mut idx = 0;

        'outer: while idx < arr.len() && !pieces.is_empty() {
            for p in 0..pieces.len() {
                if arr[idx..idx + pieces[p].len()] == pieces[p] {
                    idx += pieces[p].len();
                    pieces.remove(p);
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_can_form_array() {
        assert!(Solution::can_form_array(
            vec![15, 88],
            vec![[88].to_vec(), [15].to_vec()]
        ));
    }
}
