impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences.iter().map(|x| x.split(' ').count()).max().unwrap() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::most_words_found(vec!["alice and bob love leetcode".to_string(), "i think so too".to_string(), "this is great thanks very much".to_string()]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::most_words_found(vec!["please wait".to_string(), "continue to fight".to_string(), "continue to win".to_string()]), 3);
    }
}
