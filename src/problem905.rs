struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();
        let mut p = 0;

        for num in nums {
            if num % 2 == 0 {
                ret.insert(p, num);
                p += 1;
            } else {
                ret.push(num);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::sort_array_by_parity(vec![3,1,2,4]) => vec![2,4,3,1]);
    test_macro::test_assert_eq!(example2, Solution::sort_array_by_parity(vec![0]) => vec![0]);
}