use crate::Solution;
use std::cmp::Ordering;


// impl Solution {
//     pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//         let i = nums.partition_point(|&x| x < target);
//         i as i32
//     }
// }



impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0i32, nums.len() as i32 -1);
        while low<=high {
            let mid = low + (high-low)/2;
            match nums[mid as usize].cmp(&target){
                Ordering::Equal => {return mid;}
                Ordering::Greater => {high=mid-1;}
                Ordering::Less => {low=mid+1;}
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::Solution;

    test_macro::test_assert_eq!(example1, Solution::search_insert(vec![1,3,5,6], 5) => 2);
    test_macro::test_assert_eq!(example2, Solution::search_insert(vec![1,3,5,6], 2) => 1);
    test_macro::test_assert_eq!(example3, Solution::search_insert(vec![1,3,5,6], 7) => 4);
}