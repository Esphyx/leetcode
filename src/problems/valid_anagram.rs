use super::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut frequencies = std::collections::HashMap::new();
        for c in s.chars() {
            *frequencies.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            *frequencies.entry(c).or_insert(0) -= 1;
            if frequencies[&c] < 0 {
                return false;
            }
        }

        frequencies.values().all(|v| *v == 0)
    }
}

#[cfg(test)]
#[test]
pub fn test() {
    assert_eq!(
        Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
    assert_eq!(
        Solution::is_anagram(String::from("rat"), String::from("car")),
        false
    );
}
