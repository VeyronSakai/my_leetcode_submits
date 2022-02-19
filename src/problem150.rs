use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let a = stack.pop().unwrap().parse::<i32>().unwrap();
                    let b = stack.pop().unwrap().parse::<i32>().unwrap();
                    let c = b + a;
                    stack.push(c.to_string());
                }
                "-" => {
                    let a = stack.pop().unwrap().parse::<i32>().unwrap();
                    let b = stack.pop().unwrap().parse::<i32>().unwrap();
                    let c = b - a;
                    stack.push(c.to_string());
                }
                "*" => {
                    let a = stack.pop().unwrap().parse::<i32>().unwrap();
                    let b = stack.pop().unwrap().parse::<i32>().unwrap();
                    let c = b * a;
                    stack.push(c.to_string());
                }
                "/" => {
                    let a = stack.pop().unwrap().parse::<i32>().unwrap();
                    let b = stack.pop().unwrap().parse::<i32>().unwrap();
                    let c = b / a;
                    stack.push(c.to_string());
                }
                _ => {
                    stack.push(token);
                }
            }
        }

        stack.pop().unwrap().parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(test1,
        Solution::eval_rpn(vec!["2","10","+","3","*"].iter().map(|x| x.to_string()).collect()) => 36);

    test_macro::test_assert_eq!(test2,
        Solution::eval_rpn(vec!["4","13","5","/","+"].iter().map(|x| x.to_string()).collect()) => 6);
}