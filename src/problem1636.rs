use std::collections::{BTreeMap, HashMap};

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut mp: HashMap<i32, usize> = HashMap::new();

        for num in nums {
            mp.insert(num, mp.get(&num).unwrap_or(&0) + 1);
        }

        let mut vec = mp.iter().collect::<Vec<_>>();

        vec.sort_by(|&a, &b| {
            if a.1 == b.1 {
                b.0.cmp(a.0)
            } else {
                a.1.cmp(b.1)
            }
        });

        let mut ret = Vec::new();
        for val in vec {
            for _ in 0..*val.1 {
                ret.push(*val.0)
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]), vec![3, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]), vec![5, -1, 4, 4, -6, -6, 1, 1, 1]);
    }
}
