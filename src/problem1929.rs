use crate::Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let ans = vec![nums.clone(), nums.clone()].concat();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::get_concatenation(vec![1,2,1]) => vec![1,2,1,1,2,1]);
}