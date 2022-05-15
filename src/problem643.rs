impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut ret: f64 = f64::MIN;
        let mut cur_sum: f64 = nums[0..k].iter().sum::<i32>() as f64;
        ret = cur_sum / k as f64;

        for i in k..nums.len() {
            cur_sum += (nums[i] - nums[i - k]) as f64;
            ret = ret.max(cur_sum / k as f64);
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
        assert_eq!(Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75000);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.00000);
    }
}