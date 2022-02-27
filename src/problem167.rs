use crate::Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() - 1 {
            let result = numbers[(i + 1)..].binary_search_by(|&x| (numbers[i] + x).cmp(&target));

            match result {
                Ok(x) => {
                    return vec![(i + 1) as i32, ((i + 1) + (x + 1)) as i32];
                }
                Err(_) => {}
            }
        }
        unreachable!()
    }

    // O(N^2) super slow algorithm
    fn two_sum_slow(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() - 1 {
            for j in i + 1..numbers.len() {
                if numbers[i] + numbers[j] == target {
                    return vec![(i + 1) as i32, (j + 1) as i32];
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(exmaple1, Solution::two_sum(vec![2,7,11,15], 9) => vec![1,2]);
    test_macro::test_assert_eq!(exmaple2, Solution::two_sum(vec![2,3,4], 6) => vec![1,3]);
    test_macro::test_assert_eq!(exmaple3, Solution::two_sum(vec![-1, 0], -1) => vec![1,2]);
}
