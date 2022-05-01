use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted_scores = score.clone();
        sorted_scores.sort_by(|a, b| b.cmp(a));

        let mut mp: HashMap<i32, usize> = HashMap::new();
        for (i, &score) in sorted_scores.iter().enumerate() {
            mp.insert(score, i);
        }

        let mut ret = Vec::new();

        for num in score {
            let str = match mp.get(&num) {
                None => {
                    unreachable!()
                }
                Some(x) => {
                    match &x {
                        0 => {
                            "Gold Medal".to_string()
                        }
                        1 => {
                            "Silver Medal".to_string()
                        }
                        2 => {
                            "Bronze Medal".to_string()
                        }
                        n => {
                            (*n + 1).to_string()
                        }
                    }
                }
            };

            ret.push(str);
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
        assert_eq!(Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
                   vec!["Gold Medal".to_string(), "Silver Medal".to_string(), "Bronze Medal".to_string(), "4".to_string(), "5".to_string()]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
                   vec!["Gold Medal".to_string(), "5".to_string(), "Bronze Medal".to_string(), "Silver Medal".to_string(), "4".to_string()]);
    }
}