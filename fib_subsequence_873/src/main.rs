fn main() {
    dbg!(Solution::len_longest_fib_subseq(vec![
        1, 2, 3, 4, 5, 6, 7, 8
    ]));
}

struct Solution;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let s: std::collections::HashSet<i32> = arr.clone().into_iter().collect();
        let mut seen: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
        let mut fib_len = 0;

        for idx1 in 0..arr.len() {
            let x1 = arr[idx1];

            for x2 in arr.iter().skip(idx1 + 1) {
                let mut f1 = x1;
                let mut f2 = *x2;
                let mut length = 2;

                while s.contains(&(f1 + f2)) && !seen.contains(&(f1, f2)) {
                    seen.insert((f1, f2));
                    let next = f1 + f2;
                    f1 = f2;
                    f2 = next;

                    length += 1;
                }
                fib_len = fib_len.max(length);
            }
        }
        if fib_len > 2 {
            fib_len
        } else {
            0
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_len_longest_fib_subseq() {
        assert_eq!(
            Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            5
        );
    }
}
