struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;

        for x in nums {
            if x > 0 {
                sign *= 1;
            } else if x == 0 {
                sign = 0;
            } else {
                sign *= -1;
            }
        }

        sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
