use std::collections::HashSet;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();

        for x in arr {
            set.insert(x);
        }

        let mut count = 0;
        let mut num = 0;

        while count < k {
            num += 1;

            if !set.contains(&num) {
                count += 1;
            }
        }

        num
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
}
