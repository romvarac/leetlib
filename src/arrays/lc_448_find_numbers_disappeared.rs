use std::collections::HashSet;

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let size = nums.len();
    let mut seen = HashSet::new();
    for i in nums {
        seen.insert(i);
    }

    let mut result: Vec<i32> = Vec::new();
    for i in 1..=size {
        let index = i as i32;
        if seen.contains(&index) {
            continue;
        }
        result.push(index);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2])
    }
}
