impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;

        for vec in accounts {
            max = max.max(vec.iter().sum());
        }

        max
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]), 17);
    }
}