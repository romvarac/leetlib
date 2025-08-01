pub struct Solution {
    bad_vers: i32,
}

impl Solution {
    pub fn new(bad_vers: i32) -> Self {
        Self { bad_vers }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        if self.is_bad_version(1) {
            return 1;
        }
        let mut good: i64 = 1;
        let mut bad: i64 = n as i64;

        loop {
            if good + 1 == bad {
                return bad as i32;
            }
            let mid = (good + bad) / 2;
            if self.is_bad_version(mid as i32) {
                bad = mid;
                continue;
            }

            good = mid;
        }
    }

    fn is_bad_version(&self, n: i32) -> bool {
        self.bad_vers <= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let sol = Solution::new(4);
        assert_eq!(sol.first_bad_version(5), 4);
    }

    #[test]
    fn ex2() {
        let sol = Solution::new(1);
        assert_eq!(sol.first_bad_version(1), 1);
    }

    #[test]
    fn ex3() {
        let sol = Solution::new(1);
        assert_eq!(sol.first_bad_version(2), 1);
    }
}
