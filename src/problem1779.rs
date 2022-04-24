struct Solution;

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut ret = -1;
        let mut min = i32::MAX;

        for (i, p) in points.iter().enumerate() {
            if p[0] == x || p[1] == y {
                if min > (p[0] - x).abs() + (p[1] - y).abs() {
                    min = (p[0] - x).abs() + (p[1] - y).abs();
                    ret = i as i32;
                }
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
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
    }
}
