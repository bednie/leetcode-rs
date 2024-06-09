fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut n = std::collections::HashMap::new();
        for i in nums.iter() {
            n.entry(i).and_modify(|c| *c += 1).or_insert(1);
            if *n.get(&i).unwrap() == 3 {
                n.remove(&i);
            }
        }
        *n.into_keys().next().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    }
}
