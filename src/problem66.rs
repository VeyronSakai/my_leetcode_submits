struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        let mut carry: bool = false;

        for (i, digit) in digits.iter().rev().enumerate() {
            if i == 0 {
                ret.push((*digit + 1) % 10);
                carry = *digit + 1 >= 10;
                continue;
            }

            if carry {
                ret.push((*digit + 1) % 10);
                carry = digit + 1 >= 10;
            } else {
                ret.push(*digit);
                carry = false;
            }
        }

        if carry {
            ret.push(1);
        }

        ret.reverse();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::plus_one(vec![8, 9, 9, 9]), vec![9, 0, 0, 0]);
    }
}