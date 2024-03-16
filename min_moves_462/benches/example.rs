

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(args = vec![1, 2, 3])]
pub fn min_moves(nums: Vec<i32>) -> i32 {
    0
    // Solution::
}
