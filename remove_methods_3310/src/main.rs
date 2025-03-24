fn main() {
    dbg!(Solution::remaining_methods(
        4,
        1,
        vec![vec![1, 2], vec![0, 1], vec![3, 2]]
    ));
}

struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sus = HashSet::from([k]);
        let legit: HashSet<i32> = (0..n).collect();
        let mut m = HashMap::<i32, Vec<i32>>::new();
        let mut q = vec![k];

        for methods in invocations.iter() {
            let (i, j) = (methods[0], methods[1]);
            m.entry(i).or_default().push(j);
        }

        while let Some(method) = q.pop() {
            for &i in m.get(&method).unwrap_or(&vec![]) {
                if !sus.contains(&i) {
                    sus.insert(i);
                    q.push(i);
                }
            }
        }

        for method in 0..n {
            if !sus.contains(&method) {
                for &i in m.get(&method).unwrap_or(&vec![]) {
                    if sus.contains(&i) {
                        return legit.into_iter().collect();
                    }
                }
            }
        }

        legit.difference(&sus).copied().collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_remaining_methods() {
        assert_eq!(
            HashSet::from_iter(
                Solution::remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]])
                    .into_iter()
            ),
            HashSet::from([0, 1, 2, 3])
        );
    }
}
