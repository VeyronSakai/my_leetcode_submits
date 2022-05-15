use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp: HashMap<i32, bool> = HashMap::new();

        dp.insert(1, false);
        dp.insert(2, true);

        for num in 3..n + 1 {
            dp.insert(num as i32, Self::get_divs(num as i32).iter().any(|x| *dp.get(x).unwrap()));
        }

        *dp.get(&(n as i32)).unwrap()
    }

    fn get_divs(mut num: i32) -> HashSet<i32> {
        let mut x = 1;
        let mut ret: HashSet<i32> = HashSet::new();

        while x * x <= num {
            if num % x == 0 {
                ret.insert(x);
            }

            x += 1;
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
        assert_eq!(Solution::divisor_game(2), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::divisor_game(3), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::divisor_game(6), true);
    }
}