fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut left, mut right, mut tank) = (0, 0, 0);

        for i in 0..gas.len() {
            if right < 0 {
                left = i;
                right = 0;
            }
            tank += gas[i] - cost[i];
            right += gas[i] - cost[i];
        }
        if tank < 0 {
            return -1;
        }
        left as i32
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
    }
}
