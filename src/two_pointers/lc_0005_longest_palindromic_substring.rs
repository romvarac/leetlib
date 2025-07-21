pub fn longest_palindromic_substring(s: String) -> String {
    if s.len() == 1 {
        return s;
    }

    let len_string = s.len();
    let mut start_index = 0;
    let mut end_index = 0;
    let chars: Vec<char> = s.chars().collect();
    for index in 0..len_string {
        let mut left = index;
        let mut right = index;

        while right + 1 < len_string && chars[right + 1] == chars[left] {
            right += 1;
        }

        while right + 1 < len_string && left > 0 && chars[right + 1] == chars[left - 1] {
            right += 1;
            left -= 1;
        }

        if right - left > end_index - start_index {
            end_index = right;
            start_index = left;
        }
    }

    return s[start_index..=end_index].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            longest_palindromic_substring("babad".to_string()),
            "bab".to_string()
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            longest_palindromic_substring("cbbd".to_string()),
            "bb".to_string()
        )
    }

    #[test]
    fn ex3() {
        assert_eq!(
            longest_palindromic_substring("bb".to_string()),
            "bb".to_string()
        )
    }
}
