struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        let col_len = grid.len();
        let row_len = grid[0].len();

        for col in 0..col_len {
            for row in 0..row_len {
                if grid[col][row] == 1 {
                    ret += 4;
                }

                if grid[col][row] == 1 && row + 1 < row_len && grid[col][row + 1] == 1 {
                    ret -= 2;
                }

                if grid[col][row] == 1 && col + 1 < col_len && grid[col + 1][row] == 1 {
                    ret -= 2;
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
        assert_eq!(Solution::island_perimeter(vec![vec![0, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 0, 0]]), 16);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
    }
}
