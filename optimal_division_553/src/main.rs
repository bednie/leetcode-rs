fn main() {
    dbg!(Solution::optimal_division(vec![1000, 100, 10, 2]));
}

struct Solution;

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let mut ans = String::new();

        ans.push_str(&nums[0].to_string());

        if nums.len() == 1 {
            return ans;
        }

        if nums.len() == 2 {
            ans.push('/');
            ans.push_str(&nums[1].to_string());
            return ans;
        }

        ans.push_str("/(");

        for idx in 1..nums.len() - 1 {
            ans.push_str(&nums[idx].to_string());
            ans.push('/');
        }

        ans.push_str(&nums.last().unwrap().to_string());
        ans.push(')');
        ans
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_optimal_division() {
        assert_eq!(
            Solution::optimal_division(vec![1000, 100, 10, 2]),
            "1000/(100/10/2)".to_string()
        );
    }
}
