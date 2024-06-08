fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        let (mut fizz, mut buzz) = (0, 0);
        for i in 1..=n {
            fizz = i % 3;
            buzz = i % 5;
            match (fizz, buzz) {
                (0, 0) => ans.push("FizzBuzz".to_string()),
                (0, 1..) => ans.push("Fizz".to_string()),
                (1.., 0) => ans.push("Buzz".to_string()),
                (1.., 1..) => ans.push(i.to_string()),
                _ => {
                    panic!("Something went wrong")
                }
            }
        }
        ans
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(Solution::fizz_buzz(3), Vec::from(["1".to_string(),"2".into(),"Fizz".into()]));
    }
}
