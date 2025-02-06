fn main() {
    dbg!(Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        let freq = nums.iter().fold([0; 201], |mut acc, &n| {
            acc[n as usize + 100] += 1;
            acc
        });

        nums.sort_unstable_by(|&a, &b| {
            if freq[a as usize + 100] == freq[b as usize + 100] {
                b.cmp(&a)
            } else {
                freq[a as usize + 100].cmp(&freq[b as usize + 100])
            }
        });

        nums
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        assert_eq!(
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
    }
}
