use crate::Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let chars = s.chars().collect::<Vec<char>>();

        let mut str_vec: Vec<char> = vec![' '; indices.len()];

        for (i, index) in indices.iter().enumerate() {
            str_vec[*index as usize] = chars[i]
        }

        let ret = str_vec.iter().collect();

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::restore_string("codeleet".to_string(), vec![4,5,6,7,0,2,1,3]) => "leetcode".to_string());
    test_macro::test_assert_eq!(example2, Solution::restore_string("abc".to_string(), vec![0,1,2]) => "abc".to_string());
}