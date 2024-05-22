use std::{cmp::min, collections::HashMap};

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = min(k as usize, nums.len()-1);
        dbg!(&k);

        let mut freqs: HashMap<i32, usize> = HashMap::new();
        let mut start: usize = 0;
        let mut end: usize = k as usize + 1;

        for i in &nums[..=k as usize] {
            freqs
                .entry(*i)
                .and_modify(|e: &mut usize| *e += 1)
                .or_insert(1);

            if *freqs.get(i).expect("Something went wrong!") == 2 {
                dbg!(&freqs);
                return true;
            }
        }


        while end < nums.len() {
            freqs.entry(nums[start]).and_modify(|e: &mut usize| *e -= 1);
            freqs
                .entry(nums[end])
                .and_modify(|e: &mut usize| *e += 1)
                .or_insert(1);
            if *freqs.get(&nums[end]).expect("Something went wrong!") == 2 {
                dbg!(&freqs);
                return true;
            }
            start += 1;
            end += 1;
        }
        false
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn test_contains_nearby_duplicate_2() {
        assert!(!Solution::contains_nearby_duplicate(vec![1, 2], 2));
    }
}
