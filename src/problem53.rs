use std::cmp::max;
use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ret = i32::MIN;

        for i in 0..nums.len() {
            let mut tmp = 0;
            for j in i..nums.len() {
                tmp += nums[j];
                ret = max(ret, tmp);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(exmaple1, Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]) => 6);
    test_macro::test_assert_eq!(exmaple2, Solution::max_sub_array(vec![1]) => 1);
    test_macro::test_assert_eq!(exmaple3, Solution::max_sub_array(vec![5,4,-1,7,8]) => 23);
    test_macro::test_assert_eq!(exmaple4, Solution::max_sub_array(vec![-1]) => -1);
}