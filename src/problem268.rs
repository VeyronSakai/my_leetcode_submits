struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut tmp = vec![false; nums.len() + 1];

        for num in &nums {
            tmp[*num as usize] = true;
        }

        for (i, val) in tmp.iter().enumerate() {
            if !val {
                return i as i32;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
