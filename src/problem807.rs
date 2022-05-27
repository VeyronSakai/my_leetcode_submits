impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let line_max_arr = grid.iter().map(|x| *x.iter().max().unwrap()).collect::<Vec<i32>>();
        let mut col_max_arr: Vec<i32> = Vec::new();

        for i in 0..n {
            let mut tmp: Vec<i32> = Vec::new();

            for j in 0..n {
                tmp.push(grid[j][i]);
            }

            col_max_arr.push(*tmp.iter().max().unwrap());
        }

        let mut ret = 0;

        for i in 0..n {
            for j in 0..n {
                ret += line_max_arr[i].min(col_max_arr[j]) - grid[i][j];
            }
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
        assert_eq!(Solution::max_increase_keeping_skyline(vec![vec![3, 0, 8, 4], vec![2, 4, 5, 7], vec![9, 2, 6, 3], vec![0, 3, 1, 0]]), 35);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_increase_keeping_skyline(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]), 0);
    }
}