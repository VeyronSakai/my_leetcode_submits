use crate::Solution;
// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//         let mut lo = 0;
//         let mut hi: i32 = (nums.len() - 1) as i32;
//
//         while lo <= hi {
//             let mid = (lo + hi) / 2;
//             if nums[mid as usize] == target {
//                 return mid;
//             }
//             if nums[mid as usize] < target {
//                 lo = mid + 1;
//             } else {
//                 hi = mid - 1;
//             }
//         }
//         return -1;
//     }
// }

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = (nums.len() - 1) as i32;

        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid;
            }

            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test_macro::test_assert_eq!(exmaple1, Solution::search(vec![-1,0,3,5,9,12], 9) => 4);
    // test_macro::test_assert_eq!(exmaple2, Solution::search(vec![-1,0,3,5,9,12], 2) => -1);
    test_macro::test_assert_eq!(exmaple3, Solution::search(vec![5], -5) => -1);
    // test_macro::test_assert_eq!(exmaple4, Solution::search(vec![5], 5) => 0);
}