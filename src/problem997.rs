use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }

        let mut mp: HashMap<i32, usize> = HashMap::new();
        let mut graph: Vec<HashSet<i32>> = vec![HashSet::new(); n];

        for x in &trust {
            let mut count = mp.entry(x[1] - 1).or_insert(0);
            *count += 1;

            graph[(x[0] - 1) as usize].insert(x[1]);
        }

        for (&key, &value) in &mp {
            if value == n - 1 && graph[key as usize].is_empty() {
                return key + 1;
            }
        }

        -1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
    }
}
