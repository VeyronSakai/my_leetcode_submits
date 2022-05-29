impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut i = 0;
        let mut j = nums.len() - 1;

        let mut vec = Vec::new();

        while i < j {
            vec.push(nums[i] + nums[j]);
            i += 1;
            j -= 1;
        }

        *vec.iter().max().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }
}