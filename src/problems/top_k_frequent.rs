use super::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut most_frequent: Vec<(i32, i32)> = map.into_iter().collect();
        most_frequent.sort_by(|(_, a), (_, b)| b.cmp(a));
        most_frequent
            .into_iter()
            .take(k as usize)
            .map(|(v, _)| v)
            .collect()
    }
}

#[cfg(test)]
#[test]
pub fn test() {
    assert_eq!(
        Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2)
            .into_iter()
            .collect::<std::collections::HashSet<_>>(),
        vec![-1, 2]
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
    );
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)
            .into_iter()
            .collect::<std::collections::HashSet<_>>(),
        vec![1, 2]
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
    );
    assert_eq!(
        Solution::top_k_frequent(vec![1], 1)
            .into_iter()
            .collect::<std::collections::HashSet<_>>(),
        vec![1]
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
    );
}
