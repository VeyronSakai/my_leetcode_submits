pub struct Data {
    pub num: i32,
    pub index: usize,
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v: Vec<Data> = Vec::new();

        for i in 0..nums.len() {
            let d = Data {
                num: nums[i],
                index: i,
            };

            v.push(d);
        }

        v.sort_by_key(|x| x.num);

        let mut ans: Vec<i32> = Vec::new();

        for val in &v {
            let tmp = target - val.num as i32;

            if let Ok(j) = v.binary_search_by_key(&tmp, |x| x.num) {
                ans.push(val.index as i32);
                ans.push(v[j].index as i32);
                break;
            }
        }

        return ans;
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn get_inverse_tuple(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

#[cfg(test)]
mod tests {
    use test_macro::*;
    use super::{add, get_inverse_tuple, Solution};
    test_eq!(test1, add(1, 2) => 3);
    test_eq!(test2, 1 + 2 => 3);
    test_eq!(test3, add(2, 2) => 4);
    test_eq!(test4, get_inverse_tuple(1, 2) => (2, 1));
    test_eq!(test5, get_inverse_tuple(2, 3) => (3, 2));
    test_eq!(test6, get_inverse_tuple(2, 4) => (4, 2));
    test_eq!(two_sum_test1, Solution::two_sum(vec ! [2, 7, 11, 15], 9) => vec ! [0, 1]);
    test_eq!(two_sum_test2, Solution::two_sum(vec ! [2, 7, 11, 15], 9) => vec ! [0, 1]);
}

#[cfg(test)]
mod tests2 {
    use test_case::test_case;
    use super::Solution;

    #[test_case(vec ! [2, 7, 11, 15], 9 => vec ! [0, 1])]
    pub fn two_sum_test(nums: Vec<i32>, target: i32) -> Vec<i32> {
        return Solution::two_sum(nums, target);
    }
}
