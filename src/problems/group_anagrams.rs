use super::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort_unstable();
            map.entry(chars.into_iter().collect())
                .or_insert(Vec::new())
                .push(str);
        }

        map.into_values().collect()
    }
}

#[cfg(test)]
#[test]
pub fn test() {
    assert_eq!(
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ]),
        vec![
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
        ]
    );

    assert_eq!(
        Solution::group_anagrams(vec!["".to_string()]),
        vec![vec!["".to_string()]]
    );

    assert_eq!(
        Solution::group_anagrams(vec!["a".to_string()]),
        vec![vec!["a".to_string()]]
    );
}
