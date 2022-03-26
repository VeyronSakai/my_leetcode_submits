use std::cmp::{max, min};
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut mp: HashMap<i32, usize> = HashMap::new();

        // とりあえず最初のk個をmapに入れる
        let init_len = min(nums.len(), (k + 1) as usize);
        for i in 0..init_len {
            let mut count = mp.entry(nums[i]).or_insert(0);
            if *count == 1 {
                return true;
            } else {
                *count += 1;
            }
        }

        for i in init_len..nums.len() {
            let c = mp.get_mut(&nums[max(0, (i as i32 - k - 1)) as usize]).unwrap();
            *c = 0;

            let mut count = mp.entry(nums[i]).or_insert(0);
            if *count == 1 {
                return true;
            } else {
                *count += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
}