impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current = Self::convert_to_min(current);
        let correct = Self::convert_to_min(correct);

        let mut diff = correct - current;

        let mut ret = 0;
        ret += diff / 60;
        diff = diff % 60;

        ret += diff / 15;
        diff = diff % 15;

        ret += diff / 5;
        diff = diff % 5;

        ret += diff / 1;
        diff = diff % 1;

        if diff != 0 {
            panic!("invalid")
        }

        ret
    }

    fn convert_to_min(time: String) -> i32 {
        let mut hh = 0;
        let mut coefficient = 10;
        for c in time[0..2].chars() {
            hh += c.to_digit(10).unwrap() * coefficient;
            coefficient = coefficient / 10;
        }

        let mut mm = 0;
        coefficient = 10;
        for c in time[3..5].chars() {
            mm += c.to_digit(10).unwrap() * coefficient;
            coefficient = coefficient / 10;
        }

        (hh * 60 + mm) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_min_test1() {
        assert_eq!(Solution::convert_to_min("10:30".to_string()), 630);
    }

    #[test]
    fn convert_to_min_test2() {
        assert_eq!(Solution::convert_to_min("02:30".to_string()), 150);
    }

    #[test]
    fn example1() {
        assert_eq!(Solution::convert_time("02:30".to_string(), "04:35".to_string()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::convert_time("11:00".to_string(), "11:01".to_string()), 1);
    }
}