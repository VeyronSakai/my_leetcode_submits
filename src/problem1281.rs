struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (product, sum) = n.to_string().chars().map(|x| x.to_digit(10).unwrap()).fold((1, 0), |(product, sum), x| (product * x, sum + x));
        (product - sum) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}