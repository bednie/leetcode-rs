// leetcode 462: minimum moves to equal array elements ii
// Blair Ednie

struct Solution {}

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut min_moves: i32 = 0;
        let target: i32 = median(&nums).unwrap().ceil() as i32;

        for i in nums {
            min_moves += (target - i).abs()
        }

        min_moves
    }
}

pub fn median(data: &[i32]) -> Option<f32> {
    let size = data.len();
    let mut sorted_data = data.to_vec();
    sorted_data.sort_unstable();

    match size {
        even if even % 2 == 0 => {
            let fst_med = sorted_data[(even / 2) - 1];
            let snd_med = sorted_data[even / 2];

            let (fst, snd) = (fst_med, snd_med);
            Some((fst + snd) as f32 / 2.0)
        }
        odd => Some(sorted_data[odd / 2] as f32),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_array() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_array() {
        Solution::min_moves2(vec![]);
    }
}
