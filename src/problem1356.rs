impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;

        arr.sort();
        arr.sort_by_key(|&x| Self::count_bits(x));

        arr
    }

    fn count_bits(num: i32) -> usize {
        let mut num = num;
        let mut count = 0;
        while num > 0 {
            count += 1;
            num = num & (num - 1);
        }

        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bits_test1() {
        assert_eq!(Solution::count_bits(0), 0);
    }

    #[test]
    fn count_bits_test2() {
        assert_eq!(Solution::count_bits(1), 1);
    }

    #[test]
    fn count_bits_test3() {
        assert_eq!(Solution::count_bits(2), 1);
    }

    #[test]
    fn count_bits_test4() {
        assert_eq!(Solution::count_bits(3), 2);
    }

    #[test]
    fn example1() {
        assert_eq!(Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]), vec![0, 1, 2, 4, 8, 3, 5, 6, 7]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]), vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
    }
}