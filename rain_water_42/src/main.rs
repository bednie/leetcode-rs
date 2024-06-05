fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut height = height.clone();
        let mut water = 0;
        let mut bounds = vec![];
        let mut i: usize = 1;

        while i < height.len() - 1 {
            let (mut l, mut r) = (usize::MAX, usize::MAX);
            if height[i] < height[i - 1] {
                l = i - 1;
                r = i;
                while r < height.len() - 1 && height[r] < height[l] {
                    r += 1;
                }
            }
            if l != usize::MAX && r != usize::MAX && height[r] > height[r - 1] {
                bounds.push(vec![l, r]);
                i = r;
            }
            i += 1;
        }

        let mut j = height.len() - 2;
        while j > 0 && j < usize::MAX {
            let (mut l, mut r) = (usize::MAX, usize::MAX);
            if height[j] < height[j + 1] {
                r = j + 1;
                l = j;
                while l > 0 && height[l] < height[r] {
                    l -= 1;
                }
            }
            if l != usize::MAX && r != usize::MAX && height[l] > height[l + 1] {
                bounds.push(vec![l, r]);
                j = l;
            }
            j -= 1;
        }

        for b in bounds.into_iter() {
            let mut area: i32 = 0;
            let h = std::cmp::min(height[b[0]], height[b[1]]);
            for n in b[0] + 1..b[1] {
                if height[n] < h {
                    area += h - height[n];
                    height[n] = h;
                }
            }
            water += area;
        }
        water
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
