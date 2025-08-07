pub fn missing_number(nums: Vec<i32>) -> i32 {
    let size = nums.len() + 1;
    let sum = size * (size - 1) / 2;
    let curr: i32 = nums.iter().sum();
    sum as i32 - curr
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn ex3() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
