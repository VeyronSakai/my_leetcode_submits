struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        let mut i = 0;
        let mut j = 1;

        while i < nums.len() && j < nums.len() {
            if Self::can_proceed(&mut nums, &mut i) {
                i += 2;
            }

            if Self::can_proceed(&mut nums, &mut j) {
                j += 2;
            }

            if i < nums.len() && j < nums.len() && !Self::can_proceed(&mut nums, &mut i) && !Self::can_proceed(&mut nums, &mut j) {
                Self::swap(&mut nums, i, j);
            }
        }

        nums
    }

    fn can_proceed(nums: &mut Vec<i32>, v: &mut usize) -> bool {
        (*v % 2 == 0 && nums[*v] % 2 == 0) || (*v % 2 == 1 && nums[*v] % 2 == 1)
    }

    fn swap(v: &mut Vec<i32>, i: usize, j: usize) {
        let tmp = v[j];
        v[j] = v[i];
        v[i] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]), vec![4, 5, 2, 7]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }

    #[test]
    fn swap_test() {
        let mut v = vec![4, 2, 5, 7];
        Solution::swap(&mut v, 1, 2);
        assert_eq!(v, vec![4, 5, 2, 7]);
    }
}