struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ret = Vec::new();

        for i in 0..=n {
            let mut num = i;

            let mut count = 0;

            while num > 0 {
                if num & 1 == 1 {
                    count += 1;
                }

                num = num >> 1;
            }

            ret.push(count);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}