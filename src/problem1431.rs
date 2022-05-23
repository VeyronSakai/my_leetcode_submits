impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_num = candies.iter().max().unwrap().to_owned();

        let mut ret: Vec<bool> = Vec::new();

        for candie in candies {
            ret.push(candie + extra_candies >= max_num);
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
        assert_eq!(Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3), vec![true, true, true, false, true]);
    }
}