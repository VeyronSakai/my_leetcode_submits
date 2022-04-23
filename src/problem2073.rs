use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut tickets = tickets;

        let mut time = 0;

        while !tickets.is_empty() {
            for (i, x) in tickets.iter_mut().enumerate() {
                if *x > 0 {
                    *x -= 1;
                    time += 1;
                }

                if i == k && *x == 0 {
                    return time;
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
    }


    #[test]
    fn example2() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}