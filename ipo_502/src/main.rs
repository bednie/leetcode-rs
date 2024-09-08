fn main() {
    dbg!(Solution::find_maximized_capital(
        2,
        0,
        vec![1, 2, 3],
        vec![0, 1, 1]
    ));
}

struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        let mut w = w;
        let mut k = k;
        let mut add_back: Vec<(i32, i32)> = vec![];

        for (p, c) in profits.into_iter().zip(capital.into_iter()) {
            heap.push((p, c));
        }

        while let (Some((p, c)), true) = (heap.pop(), k > 0) {
            if w >= c {
                w += p;
                k -= 1;

                let mut i = 0;
                while i < add_back.len() {
                    if add_back[i].1 <= w {
                        let (a, b) = add_back.swap_remove(i);
                        heap.push((a, b));
                    } else {
                        i += 1;
                    }
                }
            } else {
                add_back.push((p, c));
            }
        }
        w
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_maximized_capital() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
    }
}
