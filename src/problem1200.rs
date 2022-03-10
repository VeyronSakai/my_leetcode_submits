struct Solution;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;

        arr.sort();

        let mut min_diff = i32::MAX;

        let mut ret: Vec<Vec<i32>> = Vec::new();

        for i in 0..arr.len() - 1 {
            let diff = arr[i + 1] - arr[i];

            if min_diff == diff {
                ret.push(vec![arr[i], arr[i + 1]]);
            }

            if diff < min_diff {
                ret.clear();

                ret.push(vec![arr[i], arr[i + 1]]);

                min_diff = diff;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::minimum_abs_difference(vec![4, 2, 1, 3]), vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]), vec![vec![1, 3]]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]), vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);
    }
}