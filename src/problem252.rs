use std::collections::HashSet;

struct Solution;

impl Solution {
    // This is worst solution. Don't refer to this!
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        let mut set = HashSet::new();

        for interval in &intervals {
            for val in interval[0]..interval[1] {
                if set.contains(&val) {
                    return false;
                }

                set.insert(val);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]), false);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::can_attend_meetings(vec![vec![7, 10], vec![2, 4]]), true);
    }
}