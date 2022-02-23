use crate::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut current = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[current] = nums[i];
                current += 1
            }
        }

        current as i32
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::remove_element(&mut vec![3,2,2,3], 3) => 2);
    test_macro::test_assert_eq!(example2, Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2) => 5);
}