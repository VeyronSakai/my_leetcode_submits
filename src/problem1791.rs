use std::collections::HashSet;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let edge1 = &edges[0];
        let edge2 = &edges[1];

        let mut x = 0;

        let vec = edge1.iter().filter(|x| edge2.contains(x)).map(|x| *x).collect::<Vec<i32>>();

        vec[0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
    }
}
