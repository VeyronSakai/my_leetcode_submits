struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let orig_color = image[sr as usize][sc as usize];
        let mut image = image;
        Solution::fill(&mut image, sr, sc, orig_color, new_color);

        image
    }

    fn fill(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, orig_color: i32, new_color: i32) {
        if orig_color == new_color {
            return;
        }

        let sr_limit: i32 = image.len() as i32 - 1;
        let sc_limit = image[0].len() as i32 - 1;

        if image[sr as usize][sc as usize] == orig_color {
            image[sr as usize][sc as usize] = new_color;
        }

        if sc + 1 <= sc_limit && image[sr as usize][sc as usize + 1] == orig_color {
            image[sr as usize][sc as usize + 1] = new_color;
            Solution::fill(image, sr, sc + 1, orig_color, new_color);
        }

        if sc - 1 >= 0 && image[sr as usize][sc as usize - 1] == orig_color {
            image[sr as usize][sc as usize - 1] = new_color;
            Solution::fill(image, sr, sc - 1, orig_color, new_color);
        }

        if sr + 1 <= sr_limit && image[sr as usize + 1][sc as usize] == orig_color {
            image[sr as usize + 1][sc as usize] = new_color;
            Solution::fill(image, sr + 1, sc, orig_color, new_color);
        }

        if sr - 1 >= 0 && image[sr as usize - 1][sc as usize] == orig_color {
            image[sr as usize - 1][sc as usize] = new_color;
            Solution::fill(image, sr - 1, sc, orig_color, new_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2), vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2), vec![vec![2, 2, 2], vec![2, 2, 2]]);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 1, 0]], 1, 1, 2), vec![vec![0, 0, 0], vec![0, 2, 0]]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1), vec![vec![0, 0, 0], vec![0, 1, 1]]);
    }
}