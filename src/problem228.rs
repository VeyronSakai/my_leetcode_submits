struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();

        let mut start = -1;

        for i in 0..nums.len() {
            if start == -1 {
                start = nums[i];
            }

            // If the next number exists and that number is equal to the current number plus one or if the current number is the last number
            if (i + 1 < nums.len() && nums[i + 1] != nums[i] + 1) || (i == nums.len() - 1) {
                if start == nums[i] {
                    ret.push(format!("{}", start));
                } else {
                    ret.push(format!("{}->{}", start, nums[i]));
                }
                start = -1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]), vec![String::from("0->2"), String::from("4->5"), String::from("7")]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]), vec!["0".to_string(), "2->4".to_string(), "6".to_string(), "8->9".to_string()]);
    }
}