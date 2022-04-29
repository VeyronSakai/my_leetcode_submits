struct SparseVector {
    nums: Vec<i32>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        SparseVector {
            nums
        }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut ret = 0;
        for i in 0..self.nums.len() {
            ret += self.nums[i] * vec.nums[i];
        }
        ret
    }
}


