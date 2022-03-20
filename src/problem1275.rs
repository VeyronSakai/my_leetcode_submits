struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut grid = [[' '; 3]; 3];

        for (i, m) in moves.iter().enumerate() {
            // Turn of A
            if i % 2 == 0 {
                grid[m[0] as usize][m[1] as usize] = 'X';
            } else { // Turn of B
                grid[m[0] as usize][m[1] as usize] = 'O';
            }
        }

        println!("{:?}", grid);

        // Judge
        // ヨコ
        for i in 0..3 {
            if grid[i][0] != ' ' && grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
                if grid[i][0] == 'X' {
                    // Aの勝ち
                    return "A".to_string();
                } else if grid[i][0] == 'O' {
                    // Bの勝ち
                    return "B".to_string();
                }

                unreachable!()
            }
        }

        // タテ
        for i in 0..3 {
            if grid[0][i] != ' ' && grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
                if grid[0][i] == 'X' {
                    return "A".to_string();
                } else if grid[0][i] == 'O' {
                    // Bの勝ち
                    return "B".to_string();
                }

                unreachable!()
            }
        }

        // ナナメ
        if grid[0][0] != ' ' && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            if grid[0][0] == 'X' {
                // Aの勝ち
                return "A".to_string();
            } else if grid[0][0] == 'O' {
                // Bの勝ち
                return "B".to_string();
            }

            unreachable!()
        }

        if grid[0][2] != ' ' && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            if grid[0][2] == 'X' {
                // Aの勝ち
                return "A".to_string();
            } else if grid[0][2] == 'O' {
                // Bの勝ち
                return "B".to_string();
            }

            unreachable!()
        }

        // Draw or Pending

        for i in 0..3 {
            for j in 0..3 {
                println!("{}",  grid[i][j]);
                if grid[i][j] == ' ' {
                    return "Pending".to_string();
                }
            }
        }

        return "Draw".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]]), "A");
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![0, 2], vec![1, 0], vec![2, 0]]), "B");
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::tictactoe(vec![vec![0, 0], vec![1, 1], vec![2, 0], vec![1, 0], vec![1, 2], vec![2, 1], vec![0, 1], vec![0, 2], vec![2, 2]]), "Draw");
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::tictactoe(vec![vec![1, 0], vec![0, 0], vec![0, 2], vec![1, 1], vec![1, 2], vec![0, 1]]), "Pending");
    }
}