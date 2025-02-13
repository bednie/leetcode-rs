fn main() {
    dbg!(Solution::build_array(vec![1, 3], 3));
}

struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut idx = 0;

        for num in 1..=n {
            result.push("Push".to_string());

            if target[idx] == num {
                idx += 1;
            } else {
                result.push("Pop".to_string());
            }

            if idx >= target.len() {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_build_array() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
    }
}
