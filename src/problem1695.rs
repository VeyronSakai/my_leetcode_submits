use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut cur = 0;
        let mut ret = 0;
        let mut mp: HashMap<i32, usize> = HashMap::new();

        while right < nums.len() {
            while mp.contains_key(&nums[right as usize]) {
                cur -= nums[left as usize];
                mp.remove(&nums[left as usize]);
                left += 1;
            }

            cur += nums[right as usize];
            mp.insert(nums[right as usize], right);
            right += 1;

            ret = ret.max(cur);
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
    }

    #[test]
    fn tes1() {
        assert_eq!(Solution::maximum_unique_subarray(vec![10000, 1, 10000, 1, 1, 1, 1, 1, 1]), 10001);
    }
}
