use crate::Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let (total_spaces_num, words) = Self::reorder_spaces_internal(text);

        let (space_num, extra_space_num) = if words.len() == 1 {
            (total_spaces_num, total_spaces_num)
        } else {
            ((total_spaces_num / (words.len() - 1), total_spaces_num % (words.len() - 1)))
        };

        let mut ans_str = "".to_string();

        for i in 0..words.len() {
            ans_str.push_str(&words[i]);

            if i != words.len() - 1 {
                ans_str.push_str(&(0..space_num).map(|_| " ").collect::<Vec<_>>().concat());
            }
        }

        ans_str.push_str(&(0..extra_space_num).map(|_| " ").collect::<Vec<_>>().concat());

        ans_str
    }

    fn reorder_spaces_internal(text: String) -> (usize, Vec<String>) {
        let mut space_num = 0;
        let mut tmp_str: Vec<char> = Vec::new();
        let mut words: Vec<String> = Vec::new();

        for char in text.chars() {
            if char == ' ' {
                space_num += 1;

                if !tmp_str.is_empty() {
                    words.push(tmp_str.iter().collect());
                    tmp_str.clear();
                }
            } else {
                tmp_str.push(char);
            }
        }

        if !tmp_str.is_empty() {
            words.push(tmp_str.iter().collect())
        }

        (space_num, words)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::reorder_spaces("  this   is  a sentence ".to_string()) => "this   is   a   sentence".to_string());
    test_macro::test_assert_eq!(example2, Solution::reorder_spaces(" practice   makes   perfect".to_string()) => "practice   makes   perfect ".to_string());

    test_macro::test_assert_eq!(test1, Solution::reorder_spaces_internal("  this   is  a sentence ".to_string()) => (9, vec!["this".to_string(), "is".to_string(), "a".to_string(), "sentence".to_string()]));
    test_macro::test_assert_eq!(test2, Solution::reorder_spaces_internal(" practice   makes   perfect".to_string()) => (7, vec!["practice".to_string(), "makes".to_string(), "perfect".to_string()]));
}

