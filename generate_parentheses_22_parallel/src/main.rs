use std::time::Instant;

struct ParenthesisIterator {
    stack: Vec<(String, u8, u8)>,
    n: u8,
}

impl ParenthesisIterator {
    fn new(n: i32) -> Self {
        let mut stack = Vec::new();
        stack.push((String::with_capacity(2 * n as usize), 0, 0));
        ParenthesisIterator { stack, n: n as u8 }
    }
}

impl Iterator for ParenthesisIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((mut current, open, close)) = self.stack.pop() {
            if open + close == self.n * 2 {
                return Some(current);
            }

            if close < open {
                let mut next = current.clone();
                next.push(')');
                self.stack.push((next, open, close + 1));
            }

            if open < self.n {
                current.push('(');
                self.stack.push((current, open + 1, close));
            }
        }
        None
    }
}

fn main() {
    let start = Instant::now();
    let n = 16;
    let iterator = ParenthesisIterator::new(n);
    let count = iterator.count();
    let duration = start.elapsed();
    println!("Time elapsed to generate {} pairs of () is: {:?}", n, duration);
    println!("Number of generated pairs: {}", count);
}