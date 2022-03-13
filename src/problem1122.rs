struct Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};

        let mut mp: HashMap<i32, usize> = HashMap::new();
        let mut arr2_set: HashSet<i32> = HashSet::new();

        for val in &arr2 {
            arr2_set.insert(*val);
        }

        let mut remain = Vec::new();

        for val in arr1 {
            if arr2_set.contains(&val) {
                let tmp = mp.get(&val).unwrap_or(&0);
                mp.insert(val, tmp + 1);
            } else {
                remain.push(val);
            }
        }

        let mut ret = Vec::new();

        for val in arr2 {
            if let Some(x) = mp.get(&val) {
                ret.append(&mut vec![val; *x]);
            }
        }

        remain.sort();

        ret.append(&mut remain);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::relative_sort_array(vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], vec![2, 1, 4, 3, 9, 6]), vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]), vec![22, 28, 8, 6, 17, 44]);
    }
}
