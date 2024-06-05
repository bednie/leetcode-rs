fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() == 1 || num_rows == 1 {
            return s;
        }
        
        let mut ans = vec!["".to_string(); 0];
        for row in 0..num_rows as usize {
            ans.push("".into());
        }

        let (mut row, mut i) = (0_isize, 1_isize);
        let mut c = s.chars();
        while let Some(n) = c.next() {
            ans[row as usize].push(n);
            row += i;
            if row == num_rows as isize - 1 || row == 0 {
                i *= -1;
            }
        }
        ans.join("")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(Solution::convert(String::from("PAYPALISHIRING"), 3), String::from("PAHNAPLSIIGYIR"));
    }
}
