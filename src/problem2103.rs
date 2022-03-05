use crate::Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods: Vec<Rod> = Vec::new();

        for i in 0..10 {
            let rod = Rod {
                id: i,
                has_blue: false,
                has_green: false,
                has_red: false,
            };

            rods.push(rod);
        }

        let chars: Vec<char> = rings.chars().collect();

        let mut i = 0;

        while i < rings.len() - 1 {
            match chars[i] {
                'R' => {
                    rods[Self::get_rod_num(&chars, i)].has_red = true;
                }
                'G' => {
                    rods[Self::get_rod_num(&chars, i)].has_green = true;
                }
                'B' => {
                    rods[Self::get_rod_num(&chars, i)].has_blue = true;
                }
                _ => {
                    unreachable!()
                }
            }
            i += 2;
        }

        let mut ret = 0;
        for rod in rods {
            if rod.has_red && rod.has_green && rod.has_blue {
                ret += 1;
            }
        }

        ret
    }

    fn get_rod_num(chars: &Vec<char>, i: usize) -> usize {
        chars[i + 1].to_digit(10).unwrap() as usize
    }
}

struct Rod {
    id: usize,
    has_blue: bool,
    has_green: bool,
    has_red: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::count_points("B0B6G0R6R0R6G9".to_string()) => 1);
    test_macro::test_assert_eq!(example2, Solution::count_points("B0R0G0R9R0B0G0".to_string()) => 1);
    test_macro::test_assert_eq!(example3, Solution::count_points("G4".to_string()) => 0);
}