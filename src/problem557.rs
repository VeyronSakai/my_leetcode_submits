use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans: Vec<char> = Vec::new();

        let mut buf = Vec::new();

        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == ' ' {
                buf.reverse();
                ans.append(&mut buf.clone());
                ans.push(' ');
                buf.clear();
            } else if i == chars.len() - 1 {
                buf.push(chars[i]);
                buf.reverse();
                ans.append(&mut buf.clone());
                break;
            } else {
                buf.push(chars[i]);
            }
        }

        ans.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    test_macro::test_assert_eq!(example1,
        Solution::reverse_words("Let's take LeetCode contest".to_string()) => "s'teL ekat edoCteeL tsetnoc".to_string());

    test_macro::test_assert_eq!(example2, Solution::reverse_words("God Ding".to_string()) => "doG gniD".to_string());
}