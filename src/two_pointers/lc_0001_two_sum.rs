/**
 * sorted array of integers
 * find two numbers such that they add up to a specific target number
 */
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = numbers.len() - 1;
    let mut expected: Vec<i32> = Vec::with_capacity(2);

    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            expected.push(left as i32 + 1);
            expected.push(right as i32 + 1);
            break;
        }

        if sum < target {
            left += 1;
            continue;
        }

        right -= 1;
    }

    expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn ex2() {
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3])
    }

    #[test]
    fn ex3() {
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2])
    }
}
