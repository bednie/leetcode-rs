fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut nn = n.clone();
        let mut m = 0;

        while nn > 0 {
            m += (nn % 10).pow(2_u32);
            nn /= 10;
        }

        match m {
            1 => true,
            m if m == n || m == 4 => false,
            _ => Solution::is_happy(m)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
    }
}