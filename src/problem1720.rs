impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        ret.push(first);

        for x in encoded {
            ret.push(ret.last().unwrap() ^ x);
        }

        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
    }
}
