impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for i in 0..mat.len() {
            ret += mat[i][i] + mat[i][mat.len() - 1 - i];
        }

        if mat.len() % 2 == 1 {
            ret -= mat[mat.len() / 2][mat.len() / 2];
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::diagonal_sum(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
            ]
        ), 25);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::diagonal_sum(
            vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
            ]), 8);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::diagonal_sum(
            vec![vec![5]]), 5);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::diagonal_sum(
            vec![
                vec![6, 3, 1, 10, 7, 4],
                vec![4, 10, 1, 9, 5, 10],
                vec![5, 5, 7, 3, 8, 5],
                vec![2, 7, 6, 4, 7, 6],
                vec![7, 9, 6, 1, 8, 5],
                vec![1, 7, 9, 5, 8, 4]]
        ), 67);
    }
}