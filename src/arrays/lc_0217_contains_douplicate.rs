use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashset = HashSet::new();
    for i in nums {
        if hashset.contains(&i) {
            return true;
        }

        hashset.insert(i);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true)
    }

    #[test]
    fn ex2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false)
    }

    #[test]
    fn ex3() {
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true)
    }
}
