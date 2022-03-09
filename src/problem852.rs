struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut before: i32 = -1;

        for (i, val) in arr.iter().enumerate() {
            if *val < before {
                return i as i32 - 1;
            }

            before = *val;
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    }
}