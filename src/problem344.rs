use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {

        //// You can also use reverse() provided by the standard library
        // s.reverse();

        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            let tmp = s[right];
            s[right] = s[left];
            s[left] = tmp;

            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];

        Solution::reverse_string(&mut s);

        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
}