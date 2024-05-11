fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        fn jump(point: usize, current_lane: i32, obstacles: &[i32]) -> i32 {
            let mut lanes = vec![1, 2, 3];
            let mut to_remove = vec![current_lane];
            if obstacles[point] != 0 {
                to_remove.push(obstacles[point]);
            } else {
                for &obstacle in obstacles[point + 2..].iter() {
                    if obstacle != 0 && obstacle != current_lane {
                        to_remove.push(obstacle);
                        break;
                    }
                }
            }
            lanes.retain(|&lane| !to_remove.contains(&lane));
            lanes[0]
        }

        let mut current_lane = 2;
        let mut jumps = 0;
        let mut point = 0;

        while point < obstacles.len() - 1 {
            if obstacles[point + 1] == current_lane {
                current_lane = jump(point, current_lane, &obstacles);
                jumps += 1;
            }
            point += 1;
        }
        jumps
    }
}
