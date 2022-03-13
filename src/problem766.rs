struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let col_len = matrix.len();
        let row_len = matrix[0].len();

        for i in 0..col_len - 1 {
            let val = matrix[i][0];

            for j in 0..row_len {
                if i + j < col_len && val != matrix[i + j][j] {
                    return false;
                }
            }
        }

        for j in 0..row_len - 1 {
            let val = matrix[0][j];

            for i in 0..col_len {
                if i + j < row_len && val != matrix[i][i + j] {
                    return false;
                }
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
        assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]), false);
    }
}