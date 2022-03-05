use crate::Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..nums.len() - 1 {
            for j in (i + 1)..nums.len() {
                if nums[i] == nums[j] {
                    ret += 1;
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::num_identical_pairs(vec![1,2,3,1,1,3]) => 4);
    test_macro::test_assert_eq!(example2, Solution::num_identical_pairs(vec![1,1,1,1]) => 6);
    test_macro::test_assert_eq!(example3, Solution::num_identical_pairs(vec![1,2,3]) => 0);
}