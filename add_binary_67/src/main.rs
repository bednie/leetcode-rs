fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_num: Vec<char> = a.chars().collect();
        let mut b_num: Vec<char> = b.chars().collect();
        let mut output: Vec<char> = vec![];
        let mut carry = '0';
        let (mut aa, mut bb) = (a_num.len() - 1, b_num.len() - 1);

        while aa >= 0 || bb >= 0 {
            match (a_num[aa], b_num[bb], carry, aa, bb) {
                ('0', '0', '0', _, 1..=10000) => output.push('0'),
                ('0', '0', '0', 1..=10000, _) => output.push('0'),
                ('0', '0', '1', _, _) => {
                    output.push('1');
                    carry = '0';
                }
                ('0', '1', '0', _, _) => {
                    output.push('1');
                }
                ('0', '1', '1', _, _) => {
                    output.push('0');
                    carry = '1';
                }
                ('1', '0', '0', _, _) => {
                    output.push('1');
                }
                ('1', '0', '1', _, _) => {
                    output.push('0');
                    carry = '1';
                }
                ('1', '1', '0', _, _) => {
                    output.push('0');
                    carry = '1';
                }
                ('1', '1', '1', _, _) => {
                    output.push('1');
                    carry = '1';
                }
                (.., '0', 0, 0) => {
                    break;
                }
                _ => {
                    panic!("Something went wrong");
                }
            }

            if aa == 0 {
                a_num[0] = '0';
            } else {
                aa -= 1;
            }

            if bb == 0 {
                b_num[0] = '0';
            } else {
                bb -= 1;
            }
        }
        if output.is_empty() {
            return 0.to_string();
        }
        output.iter().rev().collect::<String>()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_add_binary() {
        assert_eq!(
            Solution::add_binary(String::from("100"), String::from("110010")),
            String::from("110110")
        );
    }
}
