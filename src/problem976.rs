struct Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_by(|x, y| y.cmp(x));
        let mut ptr: Vec<usize> = vec![0, 1, 2];

        while ptr[0] < nums.len() && ptr[1] < nums.len() && ptr[2] < nums.len() {
            if nums[ptr[0]] - nums[ptr[1]] < nums[ptr[2]] && nums[ptr[2]] < nums[ptr[0]] + nums[ptr[1]] {
                return nums[ptr[0]] + nums[ptr[1]] + nums[ptr[2]];
            }

            let max_ptr = ptr[2];
            ptr = vec![ptr[1], max_ptr, max_ptr + 1];
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), 0);
    }
}