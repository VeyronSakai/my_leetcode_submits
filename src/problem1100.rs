use std::collections::HashMap;

impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let k = k as usize;
        if s.len() < k {
            return 0;
        }

        let mut cur_mp: HashMap<char, usize> = HashMap::new();
        let chars = s.chars().collect::<Vec<_>>();
        let mut ret = 0;

        // insert only fist k chars into map
        chars[0..k].iter().for_each(
            |&c| {
                let mut count = cur_mp.entry(c).or_insert(0);
                *count += 1;
            }
        );
        Self::update_ret_num(k, &mut cur_mp, &mut ret);

        chars[k..].iter().enumerate().for_each(
            |(i, &c)| {
                let mut count = cur_mp.get_mut(&chars[i]).unwrap();
                *count -= 1;
                if *count == 0 {
                    cur_mp.remove(&chars[i]);
                }

                let mut count = cur_mp.entry(c).or_insert(0);
                *count += 1;

                Self::update_ret_num(k, &mut cur_mp, &mut ret);
            }
        );

        ret
    }

    fn update_ret_num(k: usize, cur_mp: &mut HashMap<char, usize>, ret: &mut i32) {
        if cur_mp.keys().len() == k {
            *ret += 1;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::num_k_len_substr_no_repeats("havefunonleetcode".to_string(), 5), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::num_k_len_substr_no_repeats("home".to_string(), 5), 0);
    }
}