use rand::Rng;
use std::time::{Duration, Instant};

// Branchless implementation
fn branchless_sign(nums: &Vec<i32>) -> i32 {
    let (mut zero_bit, mut sign_bit) = (0, 0);

    for n in nums {
        zero_bit |= (*n == 0) as i32;
        sign_bit ^= (*n >> 31) & 1;
    }

    (1 - zero_bit) * (1 - 2 * sign_bit)
}

// A more traditional branching implementation
fn branching_sign(nums: &Vec<i32>) -> i32 {
    let mut negative_count = 0;
    for n in nums {
        if *n == 0 {
            return 0;
        }
        
        if *n < 0 {
            negative_count += 1;
        }
    }

    if negative_count % 2 == 0 {
        1
    } else {
        -1
    }
}

fn benchmark(name: &str, f: fn(&Vec<i32>) -> i32, data: &Vec<i32>, iterations: u32) -> Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = f(data);
    }
    let elapsed = start.elapsed();
    println!("{}: {:?}", name, elapsed);
    elapsed
}

fn main() {
    dbg!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));

    let mut rng = rand::rng();

    // Create test data
    let mut test_cases = Vec::new();

    // 1. Random mixed values
    let mut random_mixed = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        random_mixed.push(rng.random::<i32>());
    }
    test_cases.push(("Random mixed", random_mixed));

    // 2. Alternating positive/negative
    let mut alternating = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        alternating.push(if i % 2 == 0 { 1 } else { -1 });
    }
    test_cases.push(("Alternating", alternating));

    // 3. All positive
    let mut all_positive = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        all_positive.push(rng.random_range(1..=1000));
    }
    test_cases.push(("All positive", all_positive));

    // 4. All negative
    let mut all_negative = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        all_negative.push(rng.random_range(-1000..=-1));
    }
    test_cases.push(("All negative", all_negative));

    // 5. With early zero
    let mut with_early_zero = Vec::with_capacity(100_000);
    with_early_zero.push(0);
    for _ in 1..100_000 {
        with_early_zero.push(rng.random::<i32>());
    }
    test_cases.push(("Early zero", with_early_zero));

    // 6. With late zero
    let mut with_late_zero = Vec::with_capacity(100_000);
    for _ in 0..99_999 {
        // Generate non-zero random integers
        let mut value = rng.random::<i32>();
        if value == 0 {
            value = 1;
        }
        with_late_zero.push(value);
    }
    with_late_zero.push(0);
    test_cases.push(("Late zero", with_late_zero));

    // Run benchmarks for each test case
    let iterations = 100;
    for (name, data) in test_cases {
        println!("Test case: {}", name);
        let t1 = benchmark("  Branchless", branchless_sign, &data, iterations);
        let t2 = benchmark("  Branching", branching_sign, &data, iterations);
        println!("  Ratio: {:.2}\n", t2.as_secs_f64() / t1.as_secs_f64());
    }
}

struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let (mut zero_bit, mut sign_bit) = (0, 0);

        for n in nums {
            zero_bit |= (n == 0) as i32;
            sign_bit ^= (n >> 31) & 1;
        }

        (1 - zero_bit) * (1 - 2 * sign_bit)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn array_sign() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    }
}
