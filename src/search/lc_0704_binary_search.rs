use std::hint;

/**
 * nums: array which sorted in ascending order
 * target: T
 * find target in array
 * - exist: return index
 * - not found: return -1
 * =>>>> must write algorithm with O(logn)
 */
pub fn binary_search_by_lib(nums: Vec<i32>, target: i32) -> i32 {
    if let Ok(num) = nums.binary_search(&target) {
        return num as i32;
    }

    -1
}

pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let len = nums.len();
    let mut right = len;
    let mut mid = (left + right) / 2;

    if mid == 0 {
        if nums[mid] == target {
            return mid as i32;
        }
        return -1;
    }

    while nums[mid] != target {
        if left == right {
            return -1;
        }

        if nums[mid] < target {
            left = mid + 1;
        }

        if nums[mid] > target {
            right = mid - 1;
        }

        if left > right {
            return -1;
        }

        mid = (left + right) / 2;
        if mid >= len {
            return -1;
        }
    }
    mid as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(binary_search_by_lib(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(binary_search_by_lib(vec![-1, 0, 3, 5, 9, 12], 2), -1)
    }

    #[test]
    fn ex3() {
        assert_eq!(binary_search(vec![5], -5), -1)
    }

    #[test]
    fn ex4() {
        assert_eq!(binary_search(vec![5], 5), 0)
    }

    #[test]
    fn ex5() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 13), -1);
    }
}
