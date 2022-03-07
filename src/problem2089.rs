struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        let ret = nums
            .iter()
            .enumerate()
            .filter(|x| *x.1 == target)
            .map(|x| x.0 as i32)
            .collect::<Vec<_>>();

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::target_indices(vec![1,2,5,2,3], 2) => vec![1,2]);
    test_macro::test_assert_eq!(example2, Solution::target_indices(vec![1,2,5,2,3], 3) => vec![3]);
    test_macro::test_assert_eq!(example3, Solution::target_indices(vec![1,2,5,2,3], 5) => vec![4]);
}