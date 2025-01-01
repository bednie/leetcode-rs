fn main() {
    dbg!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
}

struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = (0, 0, 0);

        for b in bills {
            let mut excess = b - 5;

            match b {
                5 => change.0 += 5,
                10 => change.1 += 10,
                20 => change.2 += 20,
                _ => {}
            }

            while excess > 0 {
                if excess >= 20 && change.2 >= 20 {
                    excess -= 20;
                    change.2 -= 20;
                } else if excess >= 10 && change.1 >= 10 {
                    excess -= 10;
                    change.1 -= 10;
                } else if excess >= 5 && change.0 >= 5 {
                    excess -= 5;
                    change.0 -= 5;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_lemonade_change() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }
}
