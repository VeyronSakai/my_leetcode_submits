use crate::Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut vec = nums.iter().map(|&x| x * x).collect::<Vec<i32>>();

        vec.sort();

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::sorted_squares(vec![-4,-1,0,3,10]) => vec![0,1,9,16,100]);
    test_macro::test_assert_eq!(example2, Solution::sorted_squares(vec![-7,-3,2,3,11]) => vec![4,9,9,49,121]);
}