struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        if low % 2 == 0 && high % 2 == 0 {
            return (high - low) / 2;
        }

        return (high - low) / 2 + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_odds(3, 7), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_odds(8, 10), 1);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::count_odds(0, 10), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_odds(0, 9), 5);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::count_odds(21, 22), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::count_odds(13, 18), 3);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::count_odds(1, 10), 5);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::count_odds(0, 3), 2);
    }
}