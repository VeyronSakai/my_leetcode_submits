use crate::Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            ret.push(nums[nums[i] as usize]);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::build_array(vec![0,2,1,5,3,4]) => vec![0,1,2,4,5,3]);
    test_macro::test_assert_eq!(example2, Solution::build_array(vec![5,0,1,2,3,4]) => vec![4,5,0,1,2,3]);
}