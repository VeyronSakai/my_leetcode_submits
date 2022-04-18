struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut cumulative_vec: Vec<i32> = vec![0; 101];

        for log in logs {
            Self::add_population(&mut cumulative_vec, log[0], log[1]);
        }

        let mut max_population = 0;
        let mut max_year = 0;

        for (i, v) in cumulative_vec.iter().enumerate() {
            if max_population < *v {
                max_population = *v;
                max_year = i + 1950;
            }
        }

        max_year as i32
    }

    fn add_population(v: &mut Vec<i32>, birth: i32, death: i32) {
        let birth = birth as usize - 1950;
        let death = death as usize - 1950;

        for i in birth..death {
            v[i] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]), 1993);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::maximum_population(vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]]), 1960);
    }
}