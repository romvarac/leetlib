pub fn is_anagram(s: String, t: String) -> bool {
    let mut s: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = t.chars().collect();

    s.sort();
    t.sort();
    s == t
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exe1() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn exe2() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
