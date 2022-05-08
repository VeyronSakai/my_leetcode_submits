use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut set: HashSet<char> = HashSet::new();

        if s.len() < 3 {
            return 0;
        }

        let chars = s.chars().collect::<Vec<_>>();
        let mut queue: VecDeque<char> = VecDeque::new();

        let mut ret = 0;

        for i in 0..s.len() {
            queue.push_front(chars[i]);

            if i > 2 {
                queue.pop_back();
            }

            let mut set: HashSet<char> = HashSet::new();

            for x in &queue {
                set.insert(*x);
            }

            if set.len() == 3 {
                ret += 1;
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
        assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
    }
}
