struct Solution;

impl Solution {
    // pub fn defang_i_paddr(address: String) -> String {
    //     let mut ret: Vec<char> = Vec::new();
    //
    //     for char in address.chars() {
    //         if char == '.' {
    //             ret.push('[');
    //             ret.push('.');
    //             ret.push(']');
    //         } else {
    //             ret.push(char);
    //         }
    //     }
    //
    //     ret.iter().collect()
    // }

    pub fn defang_i_paddr(address: String) -> String {
        str::replace(&address, ".", "[.]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::defang_i_paddr("1.1.1.1".to_string()), "1[.]1[.]1[.]1".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::defang_i_paddr("255.100.50.0".to_string()), "255[.]100[.]50[.]0".to_string());
    }
}