fn main() {
    dbg!(Solution::valid_utf8(vec![235, 140, 4]));
}

struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut idx = 0_usize;

        while idx < data.len() {
            let discr = (data[idx] << 24).leading_ones() as usize;

            if idx + discr > data.len() {
                return false;
            }

            {
                match discr {
                    0 => idx += 1,
                    1 => return false,
                    2 => {
                        if (data[idx + 1] as u32) << 24 >> 30 != 2 {
                            return false;
                        }
                        idx += 2;
                    }
                    3 => {
                        if (data[idx + 1] as u32) << 24 >> 30 != 2
                            || (data[idx + 2] as u32) << 24 >> 30 != 2
                        {
                            return false;
                        }
                        idx += 3;
                    }
                    4 => {
                        if (data[idx + 1] as u32) << 24 >> 30 != 2
                            || (data[idx + 2] as u32) << 24 >> 30 != 2
                            || (data[idx + 3] as u32) << 24 >> 30 != 2
                        {
                            return false;
                        }

                        idx += 4;
                    }
                    _ => return false,
                }
            }
        }
        true
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_valid_utf8() {
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    }
}
