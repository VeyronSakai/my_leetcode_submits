impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums;

        // sort nums
        nums.sort_unstable();

        let mut ret = i32::MAX;

        for i in 0..nums.len() - k + 1 {
            ret = ret.min(nums[i + k - 1] - nums[i]);
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
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }

    // #[test]
    // fn example2() {
    //     assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    // }
}
