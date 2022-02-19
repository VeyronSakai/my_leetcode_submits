use std::collections::HashSet;
use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // "fl".to_string()

        // 方針
        // 全ての文字列を分割しまくってそれぞれの集合に入れる (O(N^2))
        // 全ての集合に共通している物を取得する (O(NlogN))

        // N個の集合を用意
        let mut sets: Vec<HashSet<String>> = Vec::with_capacity(strs.len());
        for str in strs {
            let set = Self::divide_into_set(&str);
            sets.push(set);
        }

        let tmp = Self::get_all_intersection(&sets);

        let mut ans = "".to_string();
        for t in tmp {
            if ans.len() < t.len() {
                ans = t;
            }
        }

        ans
    }

    fn divide_into_set(str: &String) -> HashSet<String> {
        let mut ret: HashSet<String> = HashSet::new();

        for i in 0..str.len() {
            let substring = &str[0..i + 1];
            ret.insert(substring.to_string());
        }

        ret
    }

    fn get_all_intersection(sets: &Vec<HashSet<String>>) -> HashSet<String> {
        let mut ret: HashSet<String> = sets[0].clone();

        for i in 1..sets.len() {
            ret = Self::get_intersection(&ret, &sets[i]);
        }

        ret
    }

    fn get_intersection(set1: &HashSet<String>, set2: &HashSet<String>) -> HashSet<String> {
        let mut ret: HashSet<String> = HashSet::new();

        for str in set1 {
            if set2.contains(str) {
                ret.insert(str.clone());
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::*;

    test_macro::test_assert_eq!(test1, Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string()]) => "flow".to_string());
    test_macro::test_assert_eq!(test2, Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]) => "fl".to_string());

    test_macro::test_assert_eq!(test3,
        Solution::divide_into_set(&"ab".to_string())
        => HashSet::from(["ab".to_string(), "a".to_string()])
    );

    test_macro::test_assert_eq!(test4, Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]) => "".to_string());

    test_macro::test_assert_eq!(test5, Solution::get_intersection(&(Solution::divide_into_set(&"dog".to_string())), &(Solution::divide_into_set(&"car".to_string()))) => HashSet::new());
}
