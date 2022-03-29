use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for num in &nums {
            if set.contains(num) {
                set.remove(num);
            } else {
                set.insert(*num);
            }
        }

        *set.iter().last().unwrap()
        
        // nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}