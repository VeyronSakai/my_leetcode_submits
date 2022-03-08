use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for num in arr {
            let count = map.get(&num).unwrap_or(&(0 as usize));
            map.insert(num, count + 1);
        }

        let mut set: HashSet<usize> = HashSet::new();

        for v in map {
            if set.contains(&v.1) {
                return false;
            }

            set.insert(v.1);
        }

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]), true);
    }
}