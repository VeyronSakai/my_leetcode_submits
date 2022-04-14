struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut n = n;
        let mut ret = 0;
        while n > 0 {
            if n & 1 == 1 {
                ret += 1;
            }

            n = n >> 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::hammingWeight(11), 3);
    }
}