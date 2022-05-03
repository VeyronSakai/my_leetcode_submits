impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;

        let flatten_vec = mat.iter().flatten().collect::<Vec<_>>();

        if flatten_vec.len() != r * c {
            return mat;
        }

        let mut ret = vec![vec![0; c]; r];

        for (i, &&val) in flatten_vec.iter().enumerate() {
            ret[i / c][i % c] = val;
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4), vec![[1, 2, 3, 4]]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4), vec![[1, 2], [3, 4]]);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 4, 1), vec![[1], [2], [3], [4]]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::matrix_reshape(vec![vec![1, 2, 3, 4]], 2, 2), vec![[1, 2], [3, 4]]);
    }
}