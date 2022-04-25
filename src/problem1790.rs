struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut s1 = s1;
        for i in 0..s1.len() {
            for j in 0..s1.len() {
                let mut chars = s1.chars().collect::<Vec<_>>().clone();
                chars.swap(i, j);

                if chars.into_iter().collect::<String>() == s2 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::are_almost_equal("bank".to_string(), "kanb".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::are_almost_equal("attack".to_string(), "defend".to_string()), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::are_almost_equal("kelb".to_string(), "kelb".to_string()), true);
    }
}
