impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut cur = 0;

        for &x in &nums {
            if x == 0 {
                ret = cur.max(ret);
                cur = 0;
            } else {
                cur += 1;
            }
        }

        ret = ret.max(cur);

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
