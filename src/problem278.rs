// The API is_bad_version is defined for you.
// is_bad_version(version:i32)-> bool;
// to call it use self.is_bad_version(version)

struct Solution {
    bad_version: i32,
}

impl Solution {
    fn new(bad_version: i32) -> Solution {
        Solution {
            bad_version
        }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {

        // LeetCodeではエラーになった
        // ローカルでテストを実行した時はとても実行時間が長かった
        // let vec = (1..=n).collect::<Vec<i32>>().clone();
        //
        // let ret = vec.partition_point(|&x| !self.is_bad_version(x));
        //
        // vec[ret]

        let mut lo: i32 = 1;
        let mut hi: i32 = n;

        while lo <= hi {
            let mid: i32 = lo + (hi - lo) / 2; // こうすればオーバーフローを防げる
            if self.is_bad_version(mid) {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        return lo;
    }

    // for test implementation
    pub fn is_bad_version(&self, version: i32) -> bool {
        return version >= self.bad_version;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let instance = Solution::new(4);
        assert_eq!(instance.first_bad_version(5), 4);
    }

    #[test]
    fn example2() {
        let instance = Solution::new(1);
        assert_eq!(instance.first_bad_version(1), 1);
    }

    #[test]
    fn example3() {
        let instance = Solution::new(1702766719);
        assert_eq!(instance.first_bad_version(2126753390), 1702766719);
    }
}