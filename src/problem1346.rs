struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {

        // count zero num
        let mut zero_count = 0;

        for x in &arr {
            if *x == 0 {
                zero_count += 1;
            }
        }

        let mut set: HashSet<i32> = HashSet::new();

        for &x in &arr {
            set.insert(2 * x);
        }

        for x in &arr {
            if set.contains(x) {
                if *x == 0 && zero_count == 1 {
                    continue;
                }

                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::check_if_exist(vec![7, 1, 14, 11]), true);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::check_if_exist(vec![3, 1, 7, 11]), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]), false);
    }
}