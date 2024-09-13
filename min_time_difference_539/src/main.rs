use stringvec::stringvec as sv;

fn main() {
    dbg!(Solution::find_min_difference(sv![
        "00:00", "04:00", "22:00"
    ]));
}

struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut min_difference: i32 = 1440;

        let mut times: Vec<i32> = time_points
            .into_iter()
            .map(|time: String| {
                &time[..2].parse::<i32>().unwrap() * 60 + time[3..].parse::<i32>().unwrap()
            })
            .collect();

        times.sort_unstable();

        times.windows(2).for_each(|i: &[i32]| {
            min_difference = min_difference.min(i[1] - i[0]);
        });

        min_difference.min((times[0] + 1440) - times[times.len() - 1])
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_min_difference() {
        assert_eq!(
            Solution::find_min_difference(sv!["00:00", "04:00", "22:00"]),
            120
        );
    }
}
