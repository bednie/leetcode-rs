fn main() {
    dbg!(Solution::nth_ugly_number(100));
}

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut uns = vec![0; n as usize];
        uns[0] = 1;
        let (mut i_2, mut i_3, mut i_5) = (0, 0, 0);
        let (mut n_2, mut n_3, mut n_5) = (2, 3, 5);

        (1..n).for_each(|num| {
            let next = n_2.min(n_3).min(n_5);
            uns[num as usize] = next;

            if next == n_2 {
                i_2 += 1;
                n_2 = uns[i_2 as usize] * 2;
            }

            if next == n_3 {
                i_3 += 1;
                n_3 = uns[i_3 as usize] * 3;
            }

            if next == n_5 {
                i_5 += 1;
                n_5 = uns[i_5 as usize] * 5;
            }
        });
        *uns.last().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
}
