struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut max_salary = i32::MIN;
        let mut min_salary = i32::MAX;
        let mut total = 0;

        for &x in &salary {
            max_salary = max_salary.max(x);
            min_salary = min_salary.min(x);
            total += x;
        }

        total -= max_salary;
        total -= min_salary;

        let ret = total as f64 / (salary.len() - 2) as f64;

        // println!("{:?}", ret);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!((Solution::average(vec![4000, 3000, 1000, 2000]) - 2500.00000).abs() < 0.00001);
    }

    #[test]
    fn example2() {
        assert!((Solution::average(vec![1000, 2000, 3000]) - 2000.00000).abs() < 0.00001);
    }
}