use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut char_mp: HashMap<char, usize> = HashMap::new();

        for c in chars.chars() {
            let mut count = char_mp.entry(c).or_insert(0);
            *count += 1;
        }

        let mut ret = 0;

        for word in words {
            let mut mp = char_mp.clone();
            let mut flag = true;
            for c in word.chars() {
                match mp.get_mut(&c) {
                    None => {
                        flag = false;
                        break;
                    }
                    Some(count) => {
                        if *count == 0 {
                            flag = false;
                            break;
                        }

                        *count -= 1;
                    }
                }
            }

            if flag {
                ret += word.len();
            }
        }

        ret as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_characters(vec!["cat".to_string(), "bt".to_string(), "hat".to_string(), "tree".to_string()], "atach".to_string()), 6);
    }
}