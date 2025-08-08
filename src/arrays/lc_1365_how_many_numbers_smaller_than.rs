pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut count: Vec<i32> = vec![0; 101];

    for i in nums.iter() {
        count[*i as usize + 1] += 1;
    }

    for i in 1..count.len() {
        count[i] += count[i - 1]
    }

    let mut expected: Vec<i32> = vec![0; nums.len()];
    for i in 0..expected.len() {
        expected[i] = count[nums[i] as usize];
    }

    expected
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        )
    }

    #[test]
    fn ex3() {
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        )
    }
}
