struct Solution;

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut image = image;

        for image in &mut image {
            let len = image.len();
            for i in 0..len / 2 {
                let l = image[i];
                let r = image[len - 1 - i];
                image[len - 1 - i] = l;
                image[i] = r;
            }

            // invert
            for img in image {
                *img = 1 - *img;
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_macro::test_assert_eq!(example1, Solution::flip_and_invert_image(vec![vec![1,1,0],vec![1,0,1],vec![0,0,0]]) => vec![vec![1,0,0],vec![0,1,0],vec![1,1,1]]);
    test_macro::test_assert_eq!(example2, Solution::flip_and_invert_image(vec![vec![1,1,0,0],vec![1,0,0,1],vec![0,1,1,1],vec![1,0,1,0]]) => vec![vec![1,1,0,0],vec![0,1,1,0],vec![0,0,0,1],vec![1,0,1,0]]);
}