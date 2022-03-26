struct Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut name_ptr = 0;

        for char in typed.chars() {
            if name_ptr < name.len() && name.chars().nth(name_ptr).unwrap() == char {
                name_ptr += 1;
                continue;
            } else {
                if name_ptr == 0 || name.chars().nth(name_ptr - 1).unwrap() != char {
                    return false;
                }
            }
        }

        return name_ptr == name.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()), false);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::is_long_pressed_name("saeed".to_string(), "ssaaeeddx".to_string()), false);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_long_pressed_name("leelee".to_string(), "lleeelee".to_string()), true);
    }
}