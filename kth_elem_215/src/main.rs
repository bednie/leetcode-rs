fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        (1..k).for_each(|_| _ = heap.pop());
        heap.pop().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
