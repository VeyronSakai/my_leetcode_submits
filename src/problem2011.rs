impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut ret = 0;
        for x in operations {
            match x.as_str() {
                "X++" | "++X" => {
                    ret += 1;
                }
                "X--" | "--X" => {
                    ret -= 1;
                }
                _ => {
                    unreachable!()
                }
            }
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
        assert_eq!(Solution::final_value_after_operations(vec!["--X".to_string(), "X++".to_string(), "X++".to_string()]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::final_value_after_operations(vec!["++X".to_string(), "++X".to_string(), "X++".to_string()]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::final_value_after_operations(vec!["X++".to_string(), "++X".to_string(), "--X".to_string(), "X--".to_string()]), 0);
    }
}
