use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        let arr_size = arr.len();

        for num in arr {
            let count = mp.entry(num).or_insert(0);
            *count += 1;
        }

        let mut vec = mp.iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut cur_size = 0;
        let mut ret = 0;

        for val in vec {
            cur_size += *(val.1);
            ret += 1;

            if cur_size >= arr_size / 2 {
                break;
            }
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::min_set_size(vec![1, 9]), 1);
    }
}
