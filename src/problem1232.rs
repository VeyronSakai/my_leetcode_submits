struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut coordinates = coordinates;

        for i in 1..coordinates.len() - 1 {
            if (coordinates[i + 1][0] - coordinates[i][0]) * (coordinates[i][1] - coordinates[i - 1][1]) != (coordinates[i][0] - coordinates[i - 1][0]) * (coordinates[i + 1][1] - coordinates[i][1]) {
                return false;
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
        assert_eq!(Solution::check_straight_line(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::check_straight_line(vec![vec![1, 1], vec![2, 2], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]]), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::check_straight_line(vec![vec![0, 0], vec![0, 1], vec![0, -1]]), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::check_straight_line(vec![vec![2, 4], vec![2, 5], vec![2, 8]]), true);
    }
}