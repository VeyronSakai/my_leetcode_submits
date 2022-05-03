impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ret: Vec<char> = Vec::new();
        let mut stack: Vec<char> = Vec::new();

        for char in command.chars() {
            stack.push(char);

            match stack.iter().collect::<String>().as_str() {
                "G" => {
                    ret.push('G');
                    stack.clear();
                }
                "()" => {
                    ret.push('o');
                    stack.clear();
                }
                "(al)" => {
                    ret.push('a');
                    ret.push('l');
                    stack.clear();
                }
                _ => {
                    continue;
                }
            }
        }

        ret.iter().collect()
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::interpret("G()(al)".to_string()), "Goal".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::interpret("G()()()()(al)".to_string()), "Gooooal".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::interpret("(al)G(al)()()G".to_string()), "alGalooG".to_string());
    }
}