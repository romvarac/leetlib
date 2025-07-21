use std::{cmp::max, collections::HashMap};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;
    let mut index_s = 0;
    let mut hashmap: HashMap<char, usize> = HashMap::new();

    for (index, c) in s.chars().enumerate() {
        if let Some(m) = hashmap.get(&c) {
            index_s = max(*m + 1, index_s);
        }

        longest = max(longest, index + 1 - index_s);
        hashmap.insert(c, index);
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
