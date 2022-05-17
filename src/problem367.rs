impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut left: i64 = 1;
        let mut right: i64 = num;

        if num == 1 {
            return true;
        }

        while left <= right {
            let mid = right + (left - right) / 2;

            if mid * mid < num {
                left = mid + 1;
            } else if mid * mid > num {
                right = mid - 1;
            } else {
                return true;
            }
        }

        return false;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_perfect_square(16), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_perfect_square(14), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::is_perfect_square(9), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_perfect_square(2147483647), false);
    }
}