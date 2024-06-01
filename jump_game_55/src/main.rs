fn main() {
    println!("Hello, world!");
}

struct Solution;
struct Solution2;

// first attempt
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let end = nums.len() - 1;
        let mut seen = std::collections::HashSet::new();
        seen.insert(0 as usize);
        let mut reachable = vec![0_usize];

        while let Some(top) = reachable.pop() {
            if top as usize >= end {
                return true;
            }

            seen.insert(top as usize);

            if nums[top as usize] != 0 {
                for i in top as usize + 1..top as usize + 1 + nums[top as usize] as usize {
                    if !seen.contains(&i) {
                        reachable.push(i);
                    }
                }
            }
        }
        false
    }
}

// optimized
impl Solution2 {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let end = nums.len() - 1;
        let mut max_reachable: usize = 0;

        for (i, num) in nums.into_iter().enumerate() {
            // index is not reachable
            if i > max_reachable {
                return false;
            }
            // update max reachable
            max_reachable = std::cmp::max(max_reachable, i + num as usize);
            if max_reachable >= end {
                return true;
            }
        }
        true
    }
}
