use std::collections::HashMap;

impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mp: HashMap<i32, usize> = nums2.iter().enumerate().map(|(i, &val)| (val, i)).into_iter().collect();
        nums1.iter().map(|x| *mp.get(x).unwrap() as i32).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::anagram_mappings(vec![12, 28, 46, 32, 50], vec![50, 12, 32, 46, 28]), vec![1, 4, 3, 2, 0]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::anagram_mappings(vec![84, 46], vec![84, 46]), vec![0, 1]);
    }
}