use std::collections::VecDeque;

struct MovingAverage {
    size: i32,
    queue: VecDeque<i32>,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            size,
            queue: VecDeque::with_capacity(size as usize),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() == self.size as usize {
            self.queue.pop_front();
        }

        self.queue.push_back(val);

        self.queue.iter().sum::<i32>() as f64 / self.queue.len() as f64
    }
}
