struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones.clone());

        while heap.len() > 1 {
            let max1 = heap.pop();
            let max2 = heap.pop();

            if max1.as_ref().unwrap() == max2.as_ref().unwrap() {
                continue;
            } else {
                heap.push(max1.as_ref().unwrap() - max2.as_ref().unwrap());
            }
        }


        heap.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }
}