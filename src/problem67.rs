struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_vec = a.chars().map(|x| x.to_digit(10).unwrap() as u128).collect::<Vec<u128>>();
        let b_vec = b.chars().map(|x| x.to_digit(10).unwrap() as u128).collect::<Vec<u128>>();

        let ans_num = Self::sum_binary(&a_vec) + Self::sum_binary(&b_vec);

        format!("{:b}", ans_num)
    }

    fn sum_binary(vec: &Vec<u128>) -> u128 {
        let mut tmp = 1;
        let mut ret = 0;

        for v in vec.iter().rev() {
            ret += tmp * v;
            tmp *= 2;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::add_binary(String::from("11"), String::from("1")), String::from("100"));
    }
}