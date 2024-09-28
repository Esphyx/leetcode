mod add_two_numbers;
mod contains_duplicate;
mod group_anagrams;
mod two_sum;
mod valid_anagram;
mod top_k_frequent;

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
