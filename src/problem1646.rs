impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let n = n as usize;
        let mut nums = vec![0; n + 1];
        let mut i = 1;

        if n == 0 {
            return 0;
        }

        nums[0] = 0;
        nums[1] = 1;
        let mut max_num = nums[0].max(nums[1]);

        while 2 <= 2 * i && 2 * i <= n {
            nums[2 * i] = nums[i];
            max_num = max_num.max(nums[2 * i]);

            if 2 <= 2 * i + 1 && 2 * i + 1 <= n {
                nums[2 * i + 1] = nums[i] + nums[i + 1];
                max_num = max_num.max(nums[2 * i + 1]);
            }

            i += 1;
        }

        max_num
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::get_maximum_generated(2), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::get_maximum_generated(3), 2);
    }
}