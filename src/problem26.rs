use crate::Solution;

// Solution for using dedup()
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::remove_duplicates(&mut vec![1,1,2]) => 2 );
    test_macro::test_assert_eq!(example2, Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]) => 5 );
}