struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut ret = 0;

        for &val1 in &arr1 {
            let mut is_distant = true;

            for &val2 in &arr2 {
                if (val1 - val2).abs() <= d {
                    is_distant = false;
                }
            }

            if is_distant {
                ret += 1;
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
        assert_eq!(Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2), 2);
    }
}