use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new()
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        self.queue.iter().filter(|&x| *x >= t - 3000).map(|x| *x).len() as i32
    }
}
