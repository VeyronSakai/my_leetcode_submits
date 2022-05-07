struct MyHashSet {
    nums: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        MyHashSet {
            nums: vec![false; 1000000]
        }
    }

    fn add(&mut self, key: i32) {
        self.nums[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.nums[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        return self.nums[key as usize];
    }
}

/// Your MyHashSet object will be instantiated and called as such:
/// let obj = MyHashSet::new();
/// obj.add(key);
/// obj.remove(key);
/// let ret_3: bool = obj.contains(key);
///