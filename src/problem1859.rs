pub struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut res_vec = s
            .split_whitespace()
            .map(|x| {
                (
                    x[..x.len() - 1].to_string(),
                    x[x.len() - 1..].parse::<u32>().unwrap(),
                )
            })
            .collect::<Vec<(_, _)>>();

        res_vec.sort_unstable_by_key(|x| x.1);
        res_vec
            .iter()
            .map(|x| x.0.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::sort_sentence("is2 sentence4 This1 a3".to_string()) => "This is a sentence".to_string());
    test_macro::test_assert_eq!(example2, Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()) => "Me Myself and I".to_string());
}
