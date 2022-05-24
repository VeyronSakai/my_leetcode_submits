impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            ret.insert(index[i] as usize, nums[i])
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
        assert_eq!(Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]), vec![0, 4, 1, 3, 2]);
    }
}
