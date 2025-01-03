fn main() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    dbg!(Solution::flood_fill(image, 1, 1, 2));
}

struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        fn helper(img: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32, old_color: i32) {
            if img[sr as usize][sc as usize] != color {
                img[sr as usize][sc as usize] = color;

                if sc + 1 < img[0].len() as i32 && img[sr as usize][sc as usize + 1] == old_color {
                    helper(img, sr, sc + 1, color, old_color);
                }

                if sr + 1 < img.len() as i32 && img[sr as usize + 1][sc as usize] == old_color {
                    helper(img, sr + 1, sc, color, old_color);
                }

                if sc - 1 >= 0 && img[sr as usize][sc as usize - 1] == old_color {
                    helper(img, sr, sc - 1, color, old_color);
                }

                if sr - 1 >= 0 && img[sr as usize - 1][sc as usize] == old_color {
                    helper(img, sr - 1, sc, color, old_color);
                }
            }
        }

        let mut img = image.clone();
        helper(&mut img, sr, sc, color, image[sr as usize][sc as usize]);
        img
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_flood_fill() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        assert_eq!(
            Solution::flood_fill(image, 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }
}
