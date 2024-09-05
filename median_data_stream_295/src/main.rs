// With help from this solution by user 101leetcode:
// https://leetcode.com/problems/find-median-from-data-stream/solutions/696658/python-logic-explained-with-2-heaps-clean-code

fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    let ret_1 = obj.find_median();
    obj.add_num(3);
    let ret_2 = obj.find_median();
    debug_assert_eq!(ret_1, 1.5_f64);
    debug_assert_eq!(ret_2, 2_f64);
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::<Reverse<i32>>::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));

        if self.max_heap.len() < self.min_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            if let Some(&x) = self.max_heap.peek() {
                return x as f64;
            } else {
                return self.min_heap.peek().unwrap().0 as f64;
            }
        } else {
            return (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64)
                / 2_f64;
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    // Your MedianFinder object will be instantiated and called as such:
    // let obj = MedianFinder::new();
    // obj.add_num(num);
    // let ret_2: f64 = obj.find_median();
    #[test]
    fn test_median_finder() {
        // testcase: [[],[1],[2],[],[3],[]]
        // ret_2 == 2_f64
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        let ret_1 = obj.find_median();
        obj.add_num(3);
        let ret_2 = obj.find_median();
        assert_eq!(ret_1, 1.5_f64);
        assert_eq!(ret_2, 2_f64);
    }
}
