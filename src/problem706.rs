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


/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */