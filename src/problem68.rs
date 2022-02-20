use crate::Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut stack: Vec<String> = Vec::new();
        let mut ret: Vec<String> = Vec::new();

        for i in 0..words.len() {
            stack.push(words[i].clone());

            // check width
            if !Self::is_in_max_width(&stack, max_width) {
                let mut str = "".to_string();

                // pop overflown string temporarily
                stack.pop();

                for i in 0..stack.len() {

                    // get number of whitespaces per one space between words
                    let (whitespace_num, remainder) = Solution::get_whitespaces_num(&stack, max_width);

                    str += stack[i].as_str();

                    // insert whitespaces to ret if the word is not last
                    if i != (stack.len() - 1) {
                        for _ in 0..whitespace_num {
                            str += " ";
                        }

                        // 空白が均等ではない場合は空白を追加して調整する
                        if i < remainder {
                            str += " ";
                        }
                    }

                    if stack.len() == 1 {
                        for _ in 0..whitespace_num {
                            str += " ";
                        }
                    }
                }

                ret.push(str);
                stack.clear();

                // push overflown string to stack
                stack.push(words[i].clone());
            }
        }

        // 最後の行の文字列はstackに残ってるのでretに詰める
        let mut last_str = "".to_string();
        for i in 0..stack.len() {
            last_str += stack[i].as_str();

            if i != stack.len() - 1 {
                last_str += " ";
            }
        }

        // 余った部分を空白で埋める
        let last_whitespace_num = Solution::get_last_whitespace_num(&stack, max_width);
        for _ in 0..last_whitespace_num {
            last_str += " ";
        }

        ret.push(last_str);

        ret
    }

    fn get_last_whitespace_num(stack: &Vec<String>, max_width: i32) -> usize {
        let mut tmp = 0;

        for word in stack {
            tmp += word.len();
        }

        tmp += stack.len() - 1;

        return max_width as usize - tmp;
    }

    fn is_in_max_width(stack: &Vec<String>, max_width: i32) -> bool {
        let mut length = 0;

        for word in stack {
            length += word.len() as i32;
        }

        // space between words
        length += stack.len() as i32 - 1;

        return length <= max_width;
    }

    fn get_whitespaces_num(stack: &Vec<String>, max_width: i32) -> (usize, usize) {
        let mut tmp = max_width as usize;

        for word in stack {
            tmp -= word.len();
        }

        let (ret, remainder) = if stack.len() == 1 {
            (max_width as usize - stack.first().unwrap().len(), 0)
        } else {
            (tmp / (stack.len() - 1), tmp % (stack.len() - 1))
        };

        return (ret, remainder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(
        test1,
        Solution::full_justify(vec!["This", "is", "an", "example", "of", "text", "justification."].iter().map(|x| x.to_string()).collect(), 16)
        => vec!["This    is    an", "example  of text", "justification.  "].iter().map(|x| x.to_string()).collect::<Vec<String>>()
    );

    test_macro::test_assert_eq!(test2,
    Solution::is_in_max_width(&vec!["This", "is", "an", "example"].iter().map(|x| x.to_string()).collect(), 16) => false);

    test_macro::test_assert_eq!(test3,
    Solution::is_in_max_width(&vec!["This", "is", "an"].iter().map(|x| x.to_string()).collect(), 16) => true);

    test_macro::test_assert_eq!(test4,
    Solution::get_whitespaces_num(&vec!["This", "is", "an"].iter().map(|x| x.to_string()).collect(), 16) => (4, 0));

    test_macro::test_assert_eq!(test5,
    Solution::get_whitespaces_num(&vec!["example", "of", "text"].iter().map(|x| x.to_string()).collect(), 16) => (1, 1));

    test_macro::test_assert_eq!(test6,
    Solution::get_last_whitespace_num(&vec!["justification."].iter().map(|x| x.to_string()).collect(), 16) => 2);

    test_macro::test_assert_eq!(test7,
    Solution::get_last_whitespace_num(&vec!["shall", "be"].iter().map(|x| x.to_string()).collect(), 16) => 8);

    test_macro::test_assert_eq!(
        test8,
        Solution::full_justify(vec!["What","must","be","acknowledgment","shall","be"].iter().map(|x| x.to_string()).collect(), 16)
        => vec!["What   must   be", "acknowledgment  ", "shall be        "].iter().map(|x| x.to_string()).collect::<Vec<String>>()
    );

    test_macro::test_assert_eq!(
        test9,
        Solution::full_justify(vec!["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].iter().map(|x| x.to_string()).collect(), 20)
        => vec!["Science  is  what we", "understand      well", "enough to explain to", "a  computer.  Art is", "everything  else  we", "do                  "].iter().map(|x| x.to_string()).collect::<Vec<String>>()
    );

    test_macro::test_assert_eq!(test10,
    Solution::is_in_max_width(&vec!["enough","to","explain","to"].iter().map(|x| x.to_string()).collect(), 20) => true);
}