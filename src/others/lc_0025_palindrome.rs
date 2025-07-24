pub fn is_palindrome(s: String) -> bool {
    if s.len() == 1 {
        return true;
    }

    let formated_s: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    let len_s = formated_s.len();
    if len_s == 0 {
        return true;
    }

    let mut left = 0;
    let mut right = len_s - 1;

    while left < right {
        if formated_s[left] != formated_s[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(is_palindrome("race a car".to_string()), false)
    }

    #[test]
    fn ex3() {
        assert_eq!(is_palindrome(" ".to_string()), true)
    }

    #[test]
    fn ex4() {
        assert_eq!(is_palindrome(".,".to_string()), false)
    }
}
