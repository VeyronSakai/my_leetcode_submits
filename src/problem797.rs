impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        Self::dfs(0, &graph, &mut path, &mut ret);
        ret
    }

    fn dfs(cur: i32, graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        path.push(cur);
        let cur = cur as usize;
        if cur == graph.len() - 1 {
            result.push(path.to_owned());
            path.pop();
            return;
        }

        for node in &graph[cur] {
            Self::dfs(*node, &graph, path, result);
        }
        path.pop();
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]), vec![vec![0, 1, 3], vec![0, 2, 3]]);
    }
}


