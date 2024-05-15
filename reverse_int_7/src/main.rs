fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // https://users.rust-lang.org/t/how-to-convert-a-number-to-numeric-vec/10404/2
        fn number_to_vec(n: i32) -> Vec<i32> {
            let mut digits = Vec::new();
            let mut n = n;
            while n > 9 {
                digits.push(n % 10);
                n /= 10;
            }
            digits.push(n);
            digits
        }

        let mut sign = 1;
        let v: Vec<i32>;
        if x < 0 {
            sign = -1;
            v = number_to_vec(-x);
        } else {
            v = number_to_vec(x);
        }

        let mut x_rev: i32 = 0;
        for i in v {
            match x_rev.checked_mul(10) {
                None => return 0,
                Some(_x) => { x_rev *= 10 }
            }
            match x_rev.checked_add(i) {
                None => return 0,
                Some(_x) => { x_rev += i }
            }
        }
        
        match x_rev.checked_mul(sign) {
                None => 0,
                Some(_x) => x_rev * sign
        }
    }
}
