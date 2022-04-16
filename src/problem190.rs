struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
    }
}