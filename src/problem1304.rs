impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ret = Vec::new();

        if n % 2 != 0 {
            ret.push(0);
        }

        for i in 1..=n / 2 {
            ret.push(i as i32);
            ret.push(-i as i32);
        }

        ret
    }
}

struct Solution;
