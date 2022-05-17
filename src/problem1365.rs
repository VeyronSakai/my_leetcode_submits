use std::collections::HashMap;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        let mut mp: HashMap<i32, usize> = HashMap::new();

        for (i, x) in sorted_nums.iter().enumerate() {
            if !mp.contains_key(x) {
                mp.insert(*x, i);
            }
        }

        nums.iter().map(|x| *mp.get(x).unwrap() as i32).collect::<Vec<i32>>()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]), vec![4, 0, 1, 1, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]), vec![2, 1, 0, 3]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]), vec![0, 0, 0, 0]);
    }
}