fn main() {
    dbg!(Solution::number_of_lines(
        vec![
            4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10
        ],
        "bbbcccdddaaa".to_string()
    ));
}

struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines: Vec<i32> = vec![0];

        for c in s.bytes() {
            let line_length = lines.len() - 1;
            if lines[line_length] + widths[(c - 97) as usize] <= 100 {
                lines[line_length] += widths[(c - 97) as usize];
            } else {
                lines.push(widths[(c - 97) as usize]);
            }
        }

        vec![lines.len() as i32, *lines.last().unwrap()]
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_number_of_lines() {
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            ),
            vec![2, 4]
        );
    }
}
