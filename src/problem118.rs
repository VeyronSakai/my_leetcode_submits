struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }

        let mut ret: Vec<Vec<i32>> = Vec::new();

        ret.push(vec![1]);

        if num_rows == 1 {
            return ret;
        }

        ret.push(vec![1, 1]);

        for i in 2..num_rows as usize {
            let vec = Self::get_vec(&ret[i - 1]);
            ret.push(vec);
        }

        ret
    }

    fn get_vec(input: &Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        ret.push(1);

        for i in 0..input.len() - 1 {
            ret.push(input[i] + input[i + 1]);
        }

        ret.push(1);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::generate(5), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}