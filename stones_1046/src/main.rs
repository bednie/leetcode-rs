fn main() {
    dbg!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
}

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones: std::collections::BinaryHeap<i32> = stones.into_iter().collect();

        while stones.len() > 1 {
            let stone_y = stones.pop().unwrap();
            let stone_x = stones.pop().unwrap();
            if stone_x != stone_y {
                stones.push(stone_y - stone_x);
            }
        }
        stones.pop().unwrap_or(0)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_last_stone_weight() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }
}
