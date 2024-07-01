fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for i in tokens.iter() {
            match &i[..] {
                "+" => {
                    let arg_1 = stack.pop().unwrap();
                    let arg_2 = stack.pop().unwrap();
                    stack.push(arg_2 + arg_1);
                }
                "*" => {
                    let arg_1 = stack.pop().unwrap();
                    let arg_2 = stack.pop().unwrap();
                    stack.push(arg_2 * arg_1);
                }
                "-" => {
                    let arg_1 = stack.pop().unwrap();
                    let arg_2 = stack.pop().unwrap();
                    stack.push(arg_2 - arg_1);
                }
                "/" => {
                    let arg_1 = stack.pop().unwrap();
                    let arg_2 = stack.pop().unwrap();
                    stack.push(arg_2 / arg_1);
                }
                _ => {
                    if let Ok(token) = i.parse::<i32>() {
                        stack.push(token)
                    } else {
                        panic!("Something went wrong: unable to parse string to i32.");
                    }
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".into(),
                "9".into(),
                "3".into(),
                "+".into(),
                "-11".into(),
                "*".into(),
                "/".into(),
                "*".into(),
                "17".into(),
                "+".into(),
                "5".into(),
                "+".into()
            ]),
            22
        );
    }
}
