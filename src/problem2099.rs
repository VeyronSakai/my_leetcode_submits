use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut k = k as usize;
        let mut heap = BinaryHeap::new();

        for (i, &num) in nums.iter().enumerate() {
            heap.push((Reverse(num), i));

            if heap.len() > k {
                heap.pop();
            }
        }

        let mut vec = heap.iter().map(|&(x, i)| (x.0, i)).collect::<Vec<_>>();
        vec.sort_by_key(|&(x, i)| i);

        vec.iter().map(|&(x, i)| x).collect::<Vec<i32>>()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_subsequence(vec![2, 1, 3, 3], 2), vec![3, 3]);
    }
}