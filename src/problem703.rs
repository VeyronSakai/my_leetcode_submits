use std::collections::BTreeSet;

#[derive(PartialEq, Debug)]
struct KthLargest {
    k: i32,
    sorted_set: BTreeSet<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        return KthLargest {
            k,
            sorted_set: BTreeSet::from_iter(nums),
        };
    }

    fn add(&self, val: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let obj = KthLargest::new(3, vec![4, 5, 8, 2]);

        let mut s = BTreeSet::new();
        s.insert(4);
        s.insert(5);
        s.insert(8);
        s.insert(2);

        let ret = KthLargest { k: 3, sorted_set: s };

        // assert_eq!(KthLargest::new(3, vec![4, 5, 8, 2]), ret);
    }
}

// /**
//   * `&self` means the method takes an immutable reference
//   * If you need a mutable reference, change it to `&mut self` instead
//   *
// impl KthLargest {
//
//     fn new(k: i32, nums: Vec<i32>) -> Self {
//
//     }
//
//     fn add(&self, val: i32) -> i32 {
//
//     }
// }
//
// /**
//  * Your KthLargest object will be instantiated and called as such:
//  * let obj = KthLargest::new(k, nums);
//  * let ret_1: i32 = obj.add(val);
//  */