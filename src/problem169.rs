use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();

        for num in &nums {
            let tmp = mp.entry(*num).or_insert(0);
            *tmp += 1;
        }

        let mut ret = 0;
        let mut max_count = 0;

        for (&key, &val) in &mp {
            if val > max_count {
                ret = key;
                max_count = val;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::majority_element(vec![2,2,1,1,1,2,2]), 2);
    }
}