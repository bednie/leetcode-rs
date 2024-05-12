use std::{cmp, collections::HashMap};

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        fn in_range(bomb1: &[i32], bomb2: &[i32]) -> bool {
            let distance = (bomb2[0] as i64 - bomb1[0] as i64).pow(2)
                + (bomb2[1] as i64 - bomb1[1] as i64).pow(2);
            let r = bomb1[2] as i64;
            if distance <= r.pow(2) {
                return true;
            }
            false
        }

        fn dfs(
            idx: i32,
            seen: &mut Vec<i32>,
            dag: &HashMap<i32, Vec<i32>>,
            memo: &HashMap<i32, Vec<i32>>,
        ) {
            if seen.contains(&idx) {
                return;
            }
            seen.push(idx);

            for i in dag[&idx].iter() {
                if memo.contains_key(i) {
                    seen.append(&mut memo[i].clone());
                    seen.sort_unstable();
                    seen.dedup();
                } else {
                    dfs(*i, seen, dag, memo);
                }
            }
        }

        // build dag
        let mut dag: HashMap<i32, Vec<i32>> = HashMap::<i32, Vec<i32>>::new();
        let mut memo: HashMap<i32, Vec<i32>> = HashMap::<i32, Vec<i32>>::new();
        for i in 0..bombs.len() {
            let mut connected: Vec<i32> = vec![];
            for j in 0..bombs.len() {
                if i != j && in_range(&bombs[i], &bombs[j]) {
                    connected.push(j as i32);
                }
            }
            dag.insert(i as i32, connected);
        }

        let mut max_bombs = 0;
        for i in 0..bombs.len() {
            let mut seen = vec![];
            dfs(i as i32, &mut seen, &dag, &memo);
            memo.insert(i as i32, seen.clone());
            max_bombs = cmp::max(max_bombs, seen.len());
        }
        max_bombs as i32
    }
}
