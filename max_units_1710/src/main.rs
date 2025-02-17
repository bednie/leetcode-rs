fn main() {
    dbg!(Solution::maximum_units(
        vec![vec![1, 3], vec![2, 2], vec![3, 1]],
        4
    ));
}

struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        let mut boxes = 0;
        let mut units = 0;
        box_types.sort_unstable_by_key(|b| -b[1]);

        for b in box_types {
            let m = b[0].min(truck_size - boxes);

            if m < 1 {
                break;
            }

            units += m * b[1];
            boxes += m;
        }

        units
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_maximum_units() {
        assert_eq!(
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
    }
}
