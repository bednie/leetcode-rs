fn main() {
    dbg!(Solution::can_choose(
        vec![vec![1, -1, -1], vec![3, -2, 0]],
        vec![1, -1, 0, 1, -1, -1, 3, -2, 0]
    ));
}

struct Solution;

impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        if groups.is_empty() {
            return true;
        }

        if groups[0].len() > nums.len() {
            return false;
        }

        for idx in 0..=nums.len() - groups[0].len() {
            if groups[0] == nums[idx..idx + groups[0].len()] {
                return Solution::can_choose(
                    groups[1..].to_vec(),
                    nums[idx + groups[0].len()..].to_vec(),
                );
            }
        }
        false
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_can_choose() {
        assert!(Solution::can_choose(
            vec![vec![1, -1, -1], vec![3, -2, 0]],
            vec![1, -1, 0, 1, -1, -1, 3, -2, 0]
        ));

        assert!(!Solution::can_choose(
            vec![
                vec![
                    7442762, 1817586, 647172, -8353009, -9415293, 9774715, 9752900, -9207095,
                    9315568, 5791696, -4987048
                ],
                vec![6010712, 9519715, 3929457],
                vec![8528457, 3233241, 2112524],
                vec![
                    -6191238, -679376, 220879, 3120302, 4321729, -1010177, 9207934, 8647756,
                    -7789876, 109906
                ]
            ],
            vec![
                5791696, -4987048, 5791696, 7442762, 1817586, 647172, -8353009, -9415293, 9774715,
                9752900, -9207095, 9315568, 5791696, -4987048, 7442762, 7442762, 6010712, 9519715,
                3929457, 8528457, 3233241, 2112524, 2112524, -6191238, -679376, 220879, 3120302,
                4321729, -1010177, 9207934
            ]
        ));
    }
}
