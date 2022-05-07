struct MyHashMap {
    backets: Vec<i32>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            backets: vec![-1; 1000001]
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.backets[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.backets[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.backets[key as usize] = -1;
    }
}
