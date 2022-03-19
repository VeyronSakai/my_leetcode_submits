use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_map: HashMap<char, usize> = HashMap::new();

        for char in magazine.chars() {
            let mut conut = magazine_map.entry(char).or_insert(0);
            *count += 1;
        }

        for char in ransom_note.chars() {
            if let Some(num) = magazine_map.get(&char) {
                if *num == 0 {
                    return false;
                }

                magazine_map.insert(char, *num - 1);
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::can_construct(String::from("a"), String::from("b")), false);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("ab")), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("aab")), true);
    }
}
