struct Solution;

struct Input {
    value: i32,
    rank: i32,
    index: usize,
}

impl Input {
    fn new(value: i32, index: usize) -> Input {
        Input {
            value,
            rank: 0,
            index,
        }
    }
}

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut inputs: Vec<Input> = arr.iter().enumerate().map(|(i, &x)| Input::new(x, i)).collect();

        // sort inputs
        inputs.sort_by_key(|k| k.value);

        // rank
        let mut cur_rank = 0;
        let mut before_val: Option<i32> = None;
        for input in &mut inputs {
            if before_val.is_none() || before_val.unwrap() != input.value {
                cur_rank += 1;
            }

            input.rank = cur_rank;

            before_val = Some(input.value);
        }

        // sort by index
        inputs.sort_by_key(|k| k.index);

        let ret: Vec<i32> = inputs.iter().map(|x| x.rank).collect();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]), vec![5, 3, 4, 2, 8, 6, 7, 1, 3]);
    }
}