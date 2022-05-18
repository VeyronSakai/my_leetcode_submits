use std::os::macos::raw::stat;

struct MyQueue {
    stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        let mut tmp = self.stack.clone();
        tmp.reverse();
        let val = tmp.pop().unwrap();
        tmp.reverse();
        self.stack = tmp;

        return val;
    }

    fn peek(&self) -> i32 {
        *self.stack.first().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
