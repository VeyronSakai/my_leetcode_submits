impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        let mut i = 0;
        while i < nums.len() {
            let mut vec = vec![nums[i + 1]; nums[i] as usize];
            ret.append(&mut vec);

            i += 2;
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
        assert_eq!(Solution::decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    }
}