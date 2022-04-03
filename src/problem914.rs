use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut deck = deck;
        
        let mut mp: HashMap<i32, usize> = HashMap::new();

        for val in &deck {
            let mut count = mp.entry(*val).or_insert(0);
            *count += 1;
        }

        let mut vec: Vec<usize> = mp.values().map(|x| *x).collect();

        vec.sort();

        let min = vec.first().unwrap();

        if *min < 2 {
            return false;
        }

        for val in &vec {
            if *val % min != 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]), false);
    }
}