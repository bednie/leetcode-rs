fn main() {
    println!("Hello, world!");
}

struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: vec![i32::MAX],
        }
    }

    fn push(&mut self, val: i32) {
        if val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
        self.stack.push(val);
    }

    fn pop(&mut self) -> i32 {
        let x = self.stack.pop().unwrap();
        if x == *self.min.last().unwrap() {
            self.min.pop();
        }
        x
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(-3);
        let ret_3: i32 = obj.top();
        obj.pop();
        let ret_4: i32 = obj.get_min();
        obj.push(-5);
        let ret_5: i32 = obj.get_min();

        assert_eq!(ret_3, -3);
        assert_eq!(ret_4, -2);
        assert_eq!(ret_5, -5);
    }
}
