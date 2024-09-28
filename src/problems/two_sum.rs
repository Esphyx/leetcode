use super::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0, 1];
        }

        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(&value) => return vec![value, i as i32],
                None => map.insert(num, i as i32),
            };
        }

        unreachable!()
    }
}

#[cfg(test)]
#[test]
pub fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
