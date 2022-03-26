struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let mut tmp: Vec<char> = Vec::new();

        while column_number > 0 {
            let t = ((column_number - 1) % 26) as u8;

            let char_num = (t + ('A' as u8));
            tmp.push(char_num as char);
            column_number = (column_number - 1) / 26;
        }

        tmp.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::convert_to_title(26), "Z".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::convert_to_title(27), "AA".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::convert_to_title(28), "AB".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::convert_to_title(52), "AZ".to_string());
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::convert_to_title(53), "BA".to_string());
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::convert_to_title(54), "BB".to_string());
    }
}