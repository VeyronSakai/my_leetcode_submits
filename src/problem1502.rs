struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;

        arr.sort();

        let mut bef = arr[0];
        let mut gap = arr[1] - bef;

        for i in 1..arr.len() {
            if arr[i] - bef != gap {
                return false;
            }

            bef = arr[i];
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::can_make_arithmetic_progression(vec![1, 2, 4]), false);
    }
}
