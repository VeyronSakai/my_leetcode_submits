struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut late_count = 0;
        let mut absent_count = 0;

        for char in s.chars() {
            if char == 'A' {
                absent_count += 1;
            }

            if char == 'L' {
                late_count += 1;
            } else {
                late_count = 0;
            }

            if absent_count >= 2 || late_count >= 3 {
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
        assert_eq!(Solution::check_record(String::from("PPALLP")), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::check_record(String::from("PPALLL")), false);
    }
}