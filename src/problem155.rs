use std::collections::HashMap;

struct MinStack {
    stack: Vec<i32>,
    min_num: i32,
    mp: HashMap<i32, usize>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_num: i32::MAX,
            mp: HashMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let mut count = self.mp.entry(val).or_insert(0);
        *count += 1;

        self.update_min_num();
    }

    fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        let mut count = self.mp.get_mut(&val).unwrap();
        *count -= 1;

        if *count == 0 {
            self.mp.remove(&val);
        }

        self.update_min_num();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min_num
    }

    fn update_min_num(&mut self) {
        let mut tmp_min = i32::MAX;
        for (&v, _) in &mut self.mp {
            tmp_min = tmp_min.min(v);
        }

        self.min_num = tmp_min;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        // assert_eq!(stack, MinStack{})
        stack.push(0);
        // assert_eq!(stack, MinStack{})
        stack.push(-3);
        // assert_eq!(stack, MinStack{})
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        // assert_eq!(stack, MinStack{})
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }
}