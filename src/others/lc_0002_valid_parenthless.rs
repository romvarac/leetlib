/**
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 * Valid
 * - Open brackets must be closed by the same type of brackets.
 * - Open brackets must be closed in the correct order.
 * - Every close bracket has a corresponding open bracket of the same type.
 */
pub fn is_valid_parenthless(s: String) -> bool {
    let mut close_chars: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '(' => close_chars.push(')'),
            '{' => close_chars.push('}'),
            '[' => close_chars.push(']'),
            ')' | '}' | ']' => {
                if let Some(m) = close_chars.pop() {
                    if m == c {
                        continue;
                    }

                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(is_valid_parenthless(String::from("()")), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(is_valid_parenthless(String::from("()[]{}")), true);
    }

    #[test]
    fn ex3() {
        assert_eq!(is_valid_parenthless(String::from("(]")), false);
    }

    #[test]
    fn ex4() {
        assert_eq!(is_valid_parenthless(String::from("([])")), true);
    }

    #[test]
    fn ex5() {
        assert_eq!(is_valid_parenthless(String::from("([)]")), false);
    }
}
