impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        let mut students = students;

        seats.sort_unstable();
        students.sort_unstable();

        let mut ret = 0;

        for i in 0..seats.len() {
            ret += (seats[i] - students[i]).abs();
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
        assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]), 4);
    }
}
