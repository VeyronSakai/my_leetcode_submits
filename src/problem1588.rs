impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sub_arr_len = 1;
        let mut ret: i32 = 0;

        while sub_arr_len <= arr.len() {
            for i in 0..=arr.len() - sub_arr_len {
                ret += arr[i..i + sub_arr_len].iter().sum::<i32>();
            }

            sub_arr_len += 2;
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}