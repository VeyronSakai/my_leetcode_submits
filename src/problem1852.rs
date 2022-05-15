use std::collections::HashMap;

impl Solution {
    pub fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut cur_mp: HashMap<i32, usize> = HashMap::new();
        let mut ret = Vec::new();

        for i in 0..k {
            let mut count = cur_mp.entry(nums[i]).or_insert(0);
            *count += 1;
        }
        ret.push(cur_mp.keys().len() as i32);

        for i in 0..nums.len() - k {
            let mut i_count = cur_mp.entry(nums[i + k]).or_insert(0);
            *i_count += 1;

            let mut j_count = cur_mp.get_mut(&nums[i]).unwrap();
            *j_count -= 1;

            if *j_count == 0 {
                cur_mp.remove(&nums[i]);
            }

            ret.push(cur_mp.keys().len() as i32);
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
        assert_eq!(Solution::distinct_numbers(vec![1, 2, 3, 2, 2, 1, 3], 3), vec![3, 2, 2, 2, 3]);
    }
}