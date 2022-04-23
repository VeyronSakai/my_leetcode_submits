struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students: VecDeque<i32> = VecDeque::from(students);
        let mut sandwiches: VecDeque<i32> = VecDeque::from(sandwiches);

        while !students.is_empty() {
            if !students.contains(sandwiches.front().unwrap()) {
                break;
            }

            let student = students.pop_front().unwrap();

            if student == *sandwiches.front().unwrap() {
                sandwiches.pop_front();
            } else {
                students.push_back(student);
            }
        }

        sandwiches.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]), 3);
    }
}
