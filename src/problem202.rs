use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();

        return Solution::tmp(&mut set, n);
    }

    fn tmp(set: &mut HashSet<i32>, num: i32) -> bool {
        let mut a = 0i32;

        for val in num.to_string().chars() {
            let n: i32 = val.to_digit(10).unwrap() as i32;
            a += n * n;
        }

        if a == 1 {
            return true;
        }

        if set.contains(&a) {
            return false;
        }

        set.insert(a);

        return Solution::tmp(set, a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_happy(19), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_happy(2), false);
    }
}