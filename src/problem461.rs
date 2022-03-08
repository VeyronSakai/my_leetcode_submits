struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut flag = x ^ y;

        let mut ret = 0;

        while flag > 0 {
            if flag & 1 == 1 {
                ret += 1;
            }

            flag = flag >> 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}