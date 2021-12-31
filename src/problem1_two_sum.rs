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

#[test]
fn two_sum_test() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9)
    );
}