fn main() {
    dbg!(Solution::find_final_value(vec![5,3,6,1,12], 3));
}

struct Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut o = original as usize;
        let mut n = vec![0; 100_000];

        nums.into_iter().for_each(|num| n[num as usize] += 1);

        while n[o] > 0 {
            o *= 2;
        }
        o as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_final_value() {
        assert_eq!(Solution::find_final_value(vec![5,3,6,1,12], 3), 24);
    }
}
