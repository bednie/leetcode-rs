fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut v = vec![0; n as usize+1];
        (v[0], v[1]) = (1, 1);
        for i in 2..=n as usize {
            v[i] = v[i-1] + v[i-2];
        }
        *v.last().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(6), 13);
    }
}
