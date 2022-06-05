use std::collections::{HashMap, HashSet};


impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for vec in &edges {
            let mut v1 = graph.entry(vec[0]).or_insert(Vec::new());
            v1.push(vec[1]);

            let mut v2 = graph.entry(vec[1]).or_insert(Vec::new());
            v2.push(vec[0])
        }

        let mut seen = vec![false; n];

        Self::dfs(source, &graph, &mut seen);

        seen[destination as usize]
    }

    fn dfs(target: i32, graph: &HashMap<i32, Vec<i32>>, seen: &mut Vec<bool>) {
        if seen[target as usize] {
            return;
        }

        seen[target as usize] = true;

        for &x in graph.get(&target).unwrap_or(&Vec::new()) {
            Self::dfs(x, graph, seen);
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::valid_path(6, vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]], 0, 5), false);
    }
}
