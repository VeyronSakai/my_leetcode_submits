struct Solution;

/// [1,7,3,6,5,6]
/// (左から) 0, 1, 8, 11, 17, 22, 28
/// (右から) 28, 27, 20, 17, 11, 6, 0


impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = vec![0; nums.len() + 1];
        let mut right_sum = vec![0; nums.len() + 1];

        for (i, num) in nums.iter().enumerate() {
            left_sum[i + 1] = left_sum[i] + num;
        }

        for (i, num) in nums.iter().rev().enumerate() {
            right_sum[nums.len() - i - 1] = right_sum[nums.len() - i] + num;
        }

        for i in 0..nums.len() {
            if left_sum[i] == right_sum[i + 1] {
                return i as i32;
            }
        }

        println!("{:?}", left_sum);
        println!("{:?}", right_sum);

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::pivot_index(vec![-1, -1, 0, 1, 1, 0]), 5);
    }
}