use std::collections::HashSet;

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
        return vec![];
    }

    let mut result: Vec<i32> = Vec::new();
    let mut seen: HashSet<i32> = HashSet::new();
    for i in nums {
        if seen.contains(&i) {
            result.push(i);
            continue;
        }

        seen.insert(i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3])
    }

    #[test]
    fn ex2() {
        assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1])
    }

    #[test]
    fn ex3() {
        assert_eq!(find_duplicates(vec![1]), vec![])
    }
}
