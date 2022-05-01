struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let (mut odd_heap, mut even_heap) = (BinaryHeap::new(), BinaryHeap::new());

        let mut nums = num.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>();

        for num in &nums {
            if num % 2 == 0 {
                even_heap.push(*num);
            } else {
                odd_heap.push(*num);
            }
        }

        for num in &mut nums {
            if *num % 2 == 0 {
                let even_num = even_heap.pop().unwrap();
                *num = even_num;
            } else {
                let odd_num = odd_heap.pop().unwrap();
                *num = odd_num;
            }
        }

        let mut ret = 0;
        let mut ten = 1;
        for num in nums.iter().rev() {
            ret += num * ten;
            ten *= 10;
        }

        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::largest_integer(1234), 3412);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::largest_integer(65875), 87655);
    }
}